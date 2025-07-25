use lambda_runtime::{Error, run, service_fn};
use tracing_subscriber;

mod event_handler;
use event_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json().init();
    run(service_fn(function_handler)).await
}
