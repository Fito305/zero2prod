use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// # Implementation Notes
//
// We are suing `implSubscriber` as a return type to avoid having to
// spell out the actual type of the returned subscriber, which is
// indeed quite complex.
// We need to explicitly call out that the returned subscriber is
// `Send` and `Sync` to make it possible to pass it to `init_subscriber` later on.
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Sync + Send {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// Register a subscriber as global default to process span data.
// It should be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set subscriber");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
