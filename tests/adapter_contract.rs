use yaswitch::adapters::contract::{
    validate_adapter_contract, AdapterCapabilities, AdapterOutcome, ThemeAdapter,
};
use yaswitch::core::result::{ReasonCode, YaswitchError};

#[test]
fn adapter_contract_happy_path() {
    let adapter = DummyAdapter {
        id: "dummy-full",
        capabilities: AdapterCapabilities {
            can_plan: true,
            can_apply: true,
            can_verify: true,
            can_rollback: true,
            reload_supported: true,
        },
        apply_outcome: AdapterOutcome::Applied,
    };

    validate_adapter_contract(&adapter).expect("expected adapter contract to pass");
}

#[test]
fn adapter_contract_rejects_missing_capabilities() {
    let adapter = DummyAdapter {
        id: "dummy-missing",
        capabilities: AdapterCapabilities {
            can_plan: true,
            can_apply: false,
            can_verify: true,
            can_rollback: true,
            reload_supported: true,
        },
        apply_outcome: AdapterOutcome::Applied,
    };

    let error = validate_adapter_contract(&adapter)
        .expect_err("expected adapter missing capabilities to fail");
    assert_eq!(error.code(), ReasonCode::AdapterCapabilityMissing);
}

#[test]
fn adapter_contract_enforces_safe_skip_semantics() {
    let adapter = DummyAdapter {
        id: "dummy-safe-skip",
        capabilities: AdapterCapabilities {
            can_plan: true,
            can_apply: true,
            can_verify: true,
            can_rollback: true,
            reload_supported: false,
        },
        apply_outcome: AdapterOutcome::Skipped {
            reason: ReasonCode::SkipReloadUnsupported,
        },
    };

    validate_adapter_contract(&adapter).expect("expected safe-skip semantics to pass");
}

#[test]
fn adapter_contract_rejects_invalid_skip_reason() {
    let adapter = DummyAdapter {
        id: "dummy-invalid-skip",
        capabilities: AdapterCapabilities {
            can_plan: true,
            can_apply: true,
            can_verify: true,
            can_rollback: true,
            reload_supported: false,
        },
        apply_outcome: AdapterOutcome::Skipped {
            reason: ReasonCode::ThemeSchemaInvalid,
        },
    };

    let error = validate_adapter_contract(&adapter)
        .expect_err("expected invalid skip reason to fail validation");
    assert_eq!(error.code(), ReasonCode::AdapterCapabilityMissing);
}

struct DummyAdapter {
    id: &'static str,
    capabilities: AdapterCapabilities,
    apply_outcome: AdapterOutcome,
}

impl ThemeAdapter for DummyAdapter {
    fn id(&self) -> &'static str {
        self.id
    }

    fn capabilities(&self) -> AdapterCapabilities {
        self.capabilities.clone()
    }

    fn plan(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn apply(&self) -> Result<AdapterOutcome, YaswitchError> {
        Ok(self.apply_outcome.clone())
    }

    fn verify(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn rollback(&self) -> Result<(), YaswitchError> {
        Ok(())
    }
}
