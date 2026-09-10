#[cfg(feature = "settings")]
use crate::settings::settings;

/// Metrics settings.
#[cfg_attr(feature = "settings", settings(crate_path = "crate"))]
#[cfg_attr(not(feature = "settings"), derive(Clone, Default, Debug))]
pub struct MetricsSettings {
    /// How the metrics service identifier defined in `ServiceInfo` is used
    /// for this service.
    pub service_name_format: ServiceNameFormat,

    /// Whether to report optional metrics in the telemetry server.
    pub report_optional: bool,

    /// Label name used to add [`crate::ServiceInfo::version`] to every registered metric.
    ///
    /// A metric row that already uses this name with a different value is not
    /// collected.
    #[cfg(feature = "foundations-metrics-backend")]
    pub service_version_label_name: Option<String>,
}

/// Service name format.
///
/// This dictates how [`crate::ServiceInfo::name_in_metrics`]
/// should be used by the metrics system.
#[cfg_attr(feature = "settings", settings(crate_path = "crate"))]
#[cfg_attr(not(feature = "settings"), derive(Clone, Debug, Default))]
pub enum ServiceNameFormat {
    /// Use the metrics service identifier as metric prefix.
    #[default]
    MetricPrefix,
    /// Use the metrics service identifier as the value for the given label name.
    LabelWithName(String),
}
