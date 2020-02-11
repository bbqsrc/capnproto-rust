use capnp::capability::{FromTypelessPipeline, Promise, RemotePromise, Request, Response};
use capnp::traits::{Owned, Pipelined};
use core::marker::PhantomData;

pub trait SendExt<Results>
where
    Results: Pipelined + for<'a> Owned<'a> + 'static + Unpin,
{
    fn send(self) -> RemotePromise<Results>;
}

impl<Params, Results> SendExt<Results> for Request<Params, Results>
where
    Results: Pipelined + for<'a> Owned<'a> + 'static + Unpin,
    <Results as Pipelined>::Pipeline: FromTypelessPipeline,
{
    fn send(self) -> RemotePromise<Results> {
        let RemotePromise {
            promise, pipeline, ..
        } = self.hook.send();
        let typed_promise = Promise::from_future(async move {
            Ok(Response {
                hook: promise.await?.hook,
                marker: PhantomData,
            })
        });
        RemotePromise {
            promise: typed_promise,
            pipeline: FromTypelessPipeline::new(pipeline),
        }
    }
}
