//! A configured service version label must apply to every registered metric.
#![cfg(all(feature = "foundations-metrics-backend", feature = "settings"))]

use foundations::ServiceInfo;
use foundations::telemetry::metrics::{Counter, ScrapeFormat, collect_format, metrics};
use foundations::telemetry::settings::TelemetrySettings;
use foundations::telemetry::{TelemetryConfig, TelemetryContext};

const VERSION: &str = "2026.09.10-1-abcdef";

#[metrics]
mod requests {
    pub fn total() -> Counter;
}

#[tokio::test]
async fn every_metric_includes_the_service_version() {
    let _context = TelemetryContext::test();
    let service_info = ServiceInfo {
        name: "test-service",
        name_in_metrics: "test_service".to_owned(),
        version: VERSION,
        author: "Cloudflare",
        description: "Test service",
    };
    let mut settings = TelemetrySettings::default();
    settings.server.enabled = false;
    settings.metrics.service_version_label_name = Some("version".to_owned());

    foundations::telemetry::init(TelemetryConfig {
        service_info: &service_info,
        settings: &settings,
        custom_server_routes: vec![],
    })
    .expect("initialize telemetry");
    requests::total().inc();

    let output = String::from_utf8(
        collect_format(ScrapeFormat::Text { utf8_names: false }, &settings.metrics)
            .expect("collect text metrics"),
    )
    .expect("metrics are UTF-8");
    let samples: Vec<_> = output
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    assert!(!samples.is_empty());
    assert!(
        samples
            .iter()
            .all(|sample| { sample.contains(&format!(r#"version="{VERSION}""#)) }),
        "all samples must contain the service version: {output}"
    );
}
