use lambda_http::{handler, lambda, Context, IntoResponse, Request, Response};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("test");
    lambda::run(handler(hello)).await?;
    Ok(())
}

async fn hello(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    println!("aaa");
    Ok(Response::builder().status(200).body("test").expect("failed"))
}
