use actix_cors::Cors;
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{get, delete, web, App, HttpServer};

#[get("/api/cookie")]
async fn make_cookie() -> impl Responder {
    // We create a new cookie
    let cookie = Cookie::build("test_cookie", "value")
        .path("/")
        .max_age(ActixWebDuration::days(1))
        .http_only(true)
        .finish();
    // We return the cookie
    HttpResponse::Ok()
        .cookie(cookie)
        .body("Cookie has been set")
}

#[delete("/api/cookie")]
async fn delete_cookie() -> impl Responder {
    // We create a new cookie
    let cookie = Cookie::build("test_cookie", "value")
        .path("/")
        .max_age(ActixWebDuration::ZERO)
        .http_only(true)
        .finish();
    // We return the cookie
    HttpResponse::Ok()
        .cookie(cookie)
        .body("Cookie has been deleted")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    // Start http server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            // We register the API handlers
            .service(make_cookie)
            .service(delete_cookie)
            // We add support for CORS requests
            .wrap(cors)
            // limit the maximum amount of data that server will accept
            .app_data(web::JsonConfig::default().limit(4096))
    })
    .bind("127.0.0.1:9090")?
    .workers(4)
    .run()
    .await
}
