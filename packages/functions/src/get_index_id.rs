use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let _ = core::InternalService::new("ggwp");
    let id = event
        .path_parameters()
        .first("id")
        .unwrap_or("missing")
        .to_string();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("This is index /:id => {id}").into())
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
