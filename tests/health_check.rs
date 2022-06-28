use std::net::TcpListener;

#[tokio::test]
async fn health_check_test() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}
fn spawn_app() -> String {

    // function starts the server, and then spawns the async exectution to another thread

    // binding to port 0 automatically finds an available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");

    // we need to know what that port is so we can send the appropriate request in our tests
    let port = listener.local_addr().unwrap().port();


    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}