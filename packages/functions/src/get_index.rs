use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let my_service = core::InternalService::new("ggwp");
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("This is the index {}", my_service.get()).into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
