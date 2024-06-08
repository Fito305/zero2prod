use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    pub email: String,
    pub name: String,
}


pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
     HttpResponse::Ok().finish()
 }
