use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    pub email: String,
    pub name: String,
}


pub async fn subscribe(
    form: web::Form<FormData>,
    // Restrieving a connection from the application state!
    pool: web::Data<PgPool>,
    ) -> HttpResponse {
    // `Result` has two variants: `Ok` and `Err`.
    // The first fro successes, the second for failures.
    // We use a match statement to choose what to do based
    // on the outcome.
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // Using the pool as a drop-in replacement
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
 }
