use tracing::Level;


pub(crate) fn set_up_tracing(trace_level: Level) {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

    tracing_subscriber::fmt()
        // Filter what traces are displayed based on the RUST_LOG environment
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(trace_level.into())
        )
        .with_span_events(FmtSpan::FULL)
        .init();

    tracing::info!("Tracing set to {}", trace_level.as_str());
}
