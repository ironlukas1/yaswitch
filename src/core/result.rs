use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Validation,
    Compatibility,
    Safety,
    Runtime,
}

impl ErrorCategory {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Compatibility => "compatibility",
            Self::Safety => "safety",
            Self::Runtime => "runtime",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonCode {
    ThemeSchemaInvalid,
    SkipReloadUnsupported,
    SkipTargetOptional,
    RollbackApplied,
    TransactionIoFailed,
    TransactionJournalInvalid,
    TemplateKeyMissing,
    MarkerNotFoundAppendDisabled,
    AdapterCapabilityMissing,
    PlannerDryRun,
    PlannerNoActions,
    PlannerInvalidTarget,
    ReportSerializationFailed,
    PlannerReady,
    WallpaperPathMissing,
    WallpaperUnsupportedFormat,
    WallpaperStateWriteFailed,
    WallpaperCommandUnsafe,
    WallpaperDecodeFailed,
    PaletteCacheIoFailed,
    CompositorSocketUnavailable,
    PathOutsideAllowedRoot,
    PathNotAbsolute,
    HomeDirectoryMissing,
    EvidenceWriteFailed,
    DoctorUsageInvalid,
    DoctorOutputFailed,
    ThemeCycleNoThemes,
    ThemeCycleStateIoFailed,
    ShortcutInstallFailed,
    ShortcutUnsupportedCompositor,
    SkipRestartPolicy,
    TransactionLockBusy,
}

impl ReasonCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThemeSchemaInvalid => "THEME_SCHEMA_INVALID",
            Self::SkipReloadUnsupported => "SKIP_RELOAD_UNSUPPORTED",
            Self::SkipTargetOptional => "SKIP_TARGET_OPTIONAL",
            Self::RollbackApplied => "ROLLBACK_APPLIED",
            Self::TransactionIoFailed => "TRANSACTION_IO_FAILED",
            Self::TransactionJournalInvalid => "TRANSACTION_JOURNAL_INVALID",
            Self::TemplateKeyMissing => "TEMPLATE_KEY_MISSING",
            Self::MarkerNotFoundAppendDisabled => "MARKER_NOT_FOUND_APPEND_DISABLED",
            Self::AdapterCapabilityMissing => "ADAPTER_CAPABILITY_MISSING",
            Self::PlannerDryRun => "PLANNER_DRY_RUN",
            Self::PlannerNoActions => "PLANNER_NO_ACTIONS",
            Self::PlannerInvalidTarget => "PLANNER_INVALID_TARGET",
            Self::ReportSerializationFailed => "REPORT_SERIALIZATION_FAILED",
            Self::PlannerReady => "PLANNER_READY",
            Self::WallpaperPathMissing => "WALLPAPER_PATH_MISSING",
            Self::WallpaperUnsupportedFormat => "WALLPAPER_UNSUPPORTED_FORMAT",
            Self::WallpaperStateWriteFailed => "WALLPAPER_STATE_WRITE_FAILED",
            Self::WallpaperCommandUnsafe => "WALLPAPER_COMMAND_UNSAFE",
            Self::WallpaperDecodeFailed => "WALLPAPER_DECODE_FAILED",
            Self::PaletteCacheIoFailed => "PALETTE_CACHE_IO_FAILED",
            Self::CompositorSocketUnavailable => "COMPOSITOR_SOCKET_UNAVAILABLE",
            Self::PathOutsideAllowedRoot => "PATH_OUTSIDE_ALLOWED_ROOT",
            Self::PathNotAbsolute => "PATH_NOT_ABSOLUTE",
            Self::HomeDirectoryMissing => "HOME_DIRECTORY_MISSING",
            Self::EvidenceWriteFailed => "EVIDENCE_WRITE_FAILED",
            Self::DoctorUsageInvalid => "DOCTOR_USAGE_INVALID",
            Self::DoctorOutputFailed => "DOCTOR_OUTPUT_FAILED",
            Self::ThemeCycleNoThemes => "THEME_CYCLE_NO_THEMES",
            Self::ThemeCycleStateIoFailed => "THEME_CYCLE_STATE_IO_FAILED",
            Self::ShortcutInstallFailed => "SHORTCUT_INSTALL_FAILED",
            Self::ShortcutUnsupportedCompositor => "SHORTCUT_UNSUPPORTED_COMPOSITOR",
            Self::SkipRestartPolicy => "SKIP_RESTART_POLICY",
            Self::TransactionLockBusy => "TRANSACTION_LOCK_BUSY",
        }
    }

    #[must_use]
    pub const fn category(self) -> ErrorCategory {
        match self {
            Self::ThemeSchemaInvalid => ErrorCategory::Validation,
            Self::SkipReloadUnsupported => ErrorCategory::Compatibility,
            Self::SkipTargetOptional => ErrorCategory::Compatibility,
            Self::RollbackApplied => ErrorCategory::Safety,
            Self::TemplateKeyMissing => ErrorCategory::Validation,
            Self::MarkerNotFoundAppendDisabled => ErrorCategory::Validation,
            Self::AdapterCapabilityMissing => ErrorCategory::Validation,
            Self::PlannerDryRun
            | Self::PlannerNoActions
            | Self::PlannerInvalidTarget
            | Self::ReportSerializationFailed
            | Self::PlannerReady => ErrorCategory::Runtime,
            Self::WallpaperPathMissing | Self::WallpaperUnsupportedFormat => {
                ErrorCategory::Validation
            }
            Self::WallpaperStateWriteFailed
            | Self::WallpaperCommandUnsafe
            | Self::WallpaperDecodeFailed
            | Self::PaletteCacheIoFailed => ErrorCategory::Runtime,
            Self::CompositorSocketUnavailable => ErrorCategory::Runtime,
            Self::TransactionIoFailed | Self::TransactionJournalInvalid => ErrorCategory::Runtime,
            Self::PathOutsideAllowedRoot | Self::PathNotAbsolute | Self::HomeDirectoryMissing => {
                ErrorCategory::Safety
            }
            Self::EvidenceWriteFailed => ErrorCategory::Runtime,
            Self::DoctorUsageInvalid | Self::DoctorOutputFailed => ErrorCategory::Runtime,
            Self::ThemeCycleNoThemes => ErrorCategory::Validation,
            Self::ThemeCycleStateIoFailed
            | Self::ShortcutInstallFailed
            | Self::ShortcutUnsupportedCompositor
            | Self::TransactionLockBusy => ErrorCategory::Runtime,
            Self::SkipRestartPolicy => ErrorCategory::Compatibility,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YaswitchError {
    code: ReasonCode,
    message: String,
}

impl YaswitchError {
    #[must_use]
    pub fn new(code: ReasonCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn code(&self) -> ReasonCode {
        self.code
    }

    #[must_use]
    pub fn category(&self) -> ErrorCategory {
        self.code.category()
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn payload(&self) -> ErrorPayload {
        ErrorPayload {
            code: self.code.as_str().to_string(),
            category: self.category().as_str().to_string(),
            message: self.message.clone(),
        }
    }
}

impl fmt::Display for YaswitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

impl std::error::Error for YaswitchError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorPayload {
    pub code: String,
    pub category: String,
    pub message: String,
}

pub type YaswitchResult<T> = Result<T, YaswitchError>;
