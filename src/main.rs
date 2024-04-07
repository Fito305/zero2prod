use zero2prod::run; // run() in src/lib.rs

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
