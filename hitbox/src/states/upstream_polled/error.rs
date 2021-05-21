use std::fmt;

use tracing::{instrument, trace};

use crate::states::finish::Finish;
use crate::CacheError;

pub struct UpstreamPolledError {
    pub error: CacheError,
}

impl fmt::Debug for UpstreamPolledError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("UpstreamPolledError")
    }
}

impl UpstreamPolledError {
    #[instrument]
    pub fn finish<T>(self) -> Finish<T> {
        trace!("Finish");
        Finish {
            result: Err(self.error),
        }
    }
}
