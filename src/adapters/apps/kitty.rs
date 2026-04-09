use crate::adapters::contract::{AdapterCapabilities, AdapterOutcome, ThemeAdapter};
use crate::core::result::YaswitchError;

pub struct KittyAdapter;

impl ThemeAdapter for KittyAdapter {
    fn id(&self) -> &'static str {
        "kitty"
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
