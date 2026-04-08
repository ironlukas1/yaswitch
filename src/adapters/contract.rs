use crate::core::result::{ReasonCode, YaswitchError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterCapabilities {
    pub can_plan: bool,
    pub can_apply: bool,
    pub can_verify: bool,
    pub can_rollback: bool,
    pub reload_supported: bool,
}

impl AdapterCapabilities {
    pub fn validate_required(&self) -> Result<(), YaswitchError> {
        if !(self.can_plan && self.can_apply && self.can_verify && self.can_rollback) {
            return Err(YaswitchError::new(
                ReasonCode::AdapterCapabilityMissing,
                "adapter must support plan/apply/verify/rollback",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterOutcome {
    Applied,
    Skipped { reason: ReasonCode },
}

pub trait ThemeAdapter {
    fn id(&self) -> &'static str;
    fn capabilities(&self) -> AdapterCapabilities;

    fn plan(&self) -> Result<(), YaswitchError>;
    fn apply(&self) -> Result<AdapterOutcome, YaswitchError>;
    fn verify(&self) -> Result<(), YaswitchError>;
    fn rollback(&self) -> Result<(), YaswitchError>;
}

pub fn validate_adapter_contract(adapter: &dyn ThemeAdapter) -> Result<(), YaswitchError> {
    adapter.capabilities().validate_required()?;

    adapter.plan()?;
    match adapter.apply()? {
        AdapterOutcome::Applied => {}
        AdapterOutcome::Skipped { reason } => {
            if !matches!(
                reason,
                ReasonCode::SkipReloadUnsupported
                    | ReasonCode::CompositorSocketUnavailable
                    | ReasonCode::SkipRestartPolicy
            ) {
                return Err(YaswitchError::new(
                    ReasonCode::AdapterCapabilityMissing,
                    format!(
                        "adapter {} returned unsupported skip reason {}",
                        adapter.id(),
                        reason.as_str()
                    ),
                ));
            }
        }
    }
    adapter.verify()?;
    adapter.rollback()?;

    Ok(())
}
