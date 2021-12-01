use crate::traphandlers::{tls, Trap, Unwind};

#[allow(missing_docs)]
pub type SignalHandler<'a> = &'a ();

// TODO handle system exceptions (e.g. OOM) in coordination with supervisor
pub unsafe fn platform_init() {
}
