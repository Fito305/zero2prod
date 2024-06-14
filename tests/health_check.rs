use std::net::TcpListener;
use zero2prod::startup::run;
use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    // Arrange / Set up
    let address = spawn_app();
    let client = reqwest::Client::new();

    //Act / Execute
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait MUST be in scope for us to invoke 
    // `PgConnection::connect` - it is not an inherent method of the struct!
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    let body = "name=felipe%20acosta&email=felipe_acosta%40gmail.com";

    println!("{:?}", connection_string);

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch save subscription.");


     assert_eq!(saved.email, "felipe_acosta@gmail.com");
     assert_eq!(saved.name, "felipe acosta");
}


#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // This fn is an example of table-driven test also known as
    // `parametrised test`.

    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=felipe%20acosta", "missing the email"),
        ("email=felipe_acosta%40gmail.com", "missing the name"),
        ("", "missing both the name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        // Act 
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-wwww-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload
            was {}.",error_message
            );
    }
}
