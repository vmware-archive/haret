use vr::vr_fsm::{VrState, State};
use vr::VrCtx;

/// The replica has left and has told the namespace manager to shut it down.
///
/// It doesn't respond to any messages from here on out
state!(Shutdown {
    ctx: VrCtx
});

impl Shutdown {
    pub fn enter(ctx: VrCtx) -> VrState {
        Shutdown {
            ctx: ctx
        }.into()
    }
}
