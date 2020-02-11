use crate::capability::*;
use crate::traits::{Owned, Pipelined};

impl<Params, Results> Request<Params, Results>
where
    Results: Pipelined + for<'a> Owned<'a> + 'static + Unpin,
    <Results as Pipelined>::Pipeline: FromTypelessPipeline,
{
    pub fn send(self) -> RemotePromise<Results> {
        unimplemented!("can't promise nothing on no_std");
    }
}
