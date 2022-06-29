
#[tokio::main]
async fn main() {
        // call .await on the server that we just created
        let launch_result = zero2prod::run().expect("Failed to bind address");

        println!("{}", format!("Serving app on http://127.0.0.1:{}", launch_result.port));

        launch_result.server.await.expect("Server was not available..?");
}
