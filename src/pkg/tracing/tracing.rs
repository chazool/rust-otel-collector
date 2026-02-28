use crate::pkg::config::app_config;
use opentelemetry::{global, trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider},
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use tracing_core::Level;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct OtelGuard {
    pub tracer_provider: SdkTracerProvider,
    pub meter_provider: SdkMeterProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(e) = self.tracer_provider.shutdown() {
            tracing::error!("Error shutting down tracer provider: {}", e);
        }
        if let Err(e) = self.meter_provider.shutdown() {
            tracing::error!("Error shutting down meter provider: {}", e);
        }
    }
}

fn resource() -> Resource {
    let config = app_config::get_config();

    Resource::builder()
        .with_service_name(config.service_name.clone())
        .with_schema_url(
            [
                KeyValue::new(SERVICE_VERSION, config.service_version.clone()),
                KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, config.environment.clone()),
            ],
            SCHEMA_URL,
        )
        .build()
}

pub fn init_meter_provider() -> SdkMeterProvider {
    let config = app_config::get_config();
    // Keep full URI so tonic uses plaintext for http:// and TLS for https://
    let endpoint = config.otel_endpoint.clone();

    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .with_temporality(opentelemetry_sdk::metrics::Temporality::default())
        .build()
        .unwrap();

    let reader = PeriodicReader::builder(exporter)
        .with_interval(std::time::Duration::from_secs(30))
        .build();

    let stdout_reader =
        PeriodicReader::builder(opentelemetry_stdout::MetricExporter::default()).build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource())
        .with_reader(reader)
        .with_reader(stdout_reader)
        .build();

    global::set_meter_provider(meter_provider.clone());

    meter_provider
}

pub fn init_tracer_provider(otel_endpoint: String) -> SdkTracerProvider {
    // Keep full URI so tonic uses plaintext for http:// and TLS for https://
    let endpoint = otel_endpoint;

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()
        .unwrap();

    SdkTracerProvider::builder()
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build()
}

pub fn init_tracing_subscriber() -> OtelGuard {
    let config = app_config::get_config();
    let tracer_provider = init_tracer_provider(config.otel_endpoint.clone());
    let meter_provider = init_meter_provider();

    let tracer = tracer_provider.tracer(config.service_name.clone());

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(Level::INFO))
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .with_current_span(true)
                .with_span_list(true),
        )
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    OtelGuard {
        tracer_provider,
        meter_provider,
    }
}
