use crate::capability::*;
use crate::traits::{Owned, Pipelined};
use core::marker::PhantomData;

impl<Params, Results> Request<Params, Results>
where
    Results: Pipelined + for<'a> Owned<'a> + 'static + Unpin,
    <Results as Pipelined>::Pipeline: FromTypelessPipeline,
{
    pub fn send(self) -> RemotePromise<Results> {
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
