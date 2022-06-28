
#[tokio::main]
async fn main() -> std::io::Result<()> {
        // call .await on the server that we just created
        zero2prod::run()?.await
}
