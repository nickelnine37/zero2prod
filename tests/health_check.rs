#[tokio::test]
async fn health_check_test() {

    // this impl Future, but we do not await it
    let launch_result = zero2prod::run().expect("Failed to bind address");

    // send to a thread, which closes when executuion ends rather than await, which blocks
    let _ = tokio::spawn(launch_result.server);

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://127.0.0.1:{}/health_check", launch_result.port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}
