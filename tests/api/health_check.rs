//! tests/api/health_check.rs
use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    //Arrange

    //No .await is necessary here.
    let app = spawn_app().await;

    //We need to bring in `reqwest`
    //to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute requests");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
