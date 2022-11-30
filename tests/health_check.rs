use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    // using port 0 will cause the operating system to use a random port that is free
    // so that multiple tests can run concurrently (as long as we don't add state like a database to the app)
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed binding to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("address in use ?");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
