use crate::adapters::contract::{AdapterCapabilities, AdapterOutcome, ThemeAdapter};
use crate::core::result::{ReasonCode, YaswitchError};

pub struct SwayAdapter {
    socket_available: bool,
}

impl SwayAdapter {
    pub fn new(socket_available: bool) -> Self {
        Self { socket_available }
    }

    #[must_use]
    pub fn socket_available(&self) -> bool {
        self.socket_available
    }
}

impl ThemeAdapter for SwayAdapter {
    fn id(&self) -> &'static str {
        "sway"
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            can_plan: true,
            can_apply: true,
            can_verify: true,
            can_rollback: true,
            reload_supported: self.socket_available,
        }
    }

    fn plan(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn apply(&self) -> Result<AdapterOutcome, YaswitchError> {
        if self.socket_available {
            Ok(AdapterOutcome::Applied)
        } else {
            Ok(AdapterOutcome::Skipped {
                reason: ReasonCode::CompositorSocketUnavailable,
            })
        }
    }

    fn verify(&self) -> Result<(), YaswitchError> {
        if self.socket_available {
            Ok(())
        } else {
            Err(YaswitchError::new(
                ReasonCode::CompositorSocketUnavailable,
                "sway socket is unavailable for verification",
            ))
        }
    }

    fn rollback(&self) -> Result<(), YaswitchError> {
        Ok(())
    }
}
