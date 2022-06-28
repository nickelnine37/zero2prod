#[tokio::test]
async fn health_check_test() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    assert_eq!(Some(0), response.content_length());

}

fn spawn_app() {
    // function starts the server, and then spawns the async exectution to another thread
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}