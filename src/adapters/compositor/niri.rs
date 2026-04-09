use crate::adapters::contract::{AdapterCapabilities, AdapterOutcome, ThemeAdapter};
use crate::core::result::YaswitchError;

pub struct NiriAdapter;

impl ThemeAdapter for NiriAdapter {
    fn id(&self) -> &'static str {
        "niri"
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            can_plan: true,
            can_apply: true,
            can_verify: true,
            can_rollback: true,
            reload_supported: true,
        }
    }

    fn plan(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn apply(&self) -> Result<AdapterOutcome, YaswitchError> {
        Ok(AdapterOutcome::Applied)
    }

    fn verify(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn rollback(&self) -> Result<(), YaswitchError> {
        Ok(())
    }
}
