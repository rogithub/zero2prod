use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer };
use tracing_log::LogTracer;
use tracing_subscriber::{ layer::SubscriberExt, EnvFilter, Registry };
use tracing_subscriber::fmt::MakeWriter;



/// Compose multiple layers into a 'tracing's subscriber'
/// 
/// # Implementation notes
/// 
/// We are using 'impl subscriber' as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is 
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is 
/// 'Send' and 'Sync' to make it possible to pass it to 'init_subscriber'
/// latter on.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink
) -> impl Subscriber + Send + Sync 
     // This "weird" syntax is a higher-ranked trait bound (HRTB)
     // It basically means that Sink implements the 'MakeWriter' 
     // trait for all choices of the lifetime parameter ´'a´
     // checkout https://doc.rust-lang.org/nomicon/hrtb.html
     // for more details.
     where Sink: for<'a> MakeWriter<'a> + Send + Sync +'static {     
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name, 
        sink
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// It should only be called once.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set looger");
    set_global_default(subscriber).expect("Failed to set subscriber")
}