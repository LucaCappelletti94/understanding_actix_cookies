use actix_cors::Cors;
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{delete, get, web, App, HttpServer};
use commons::QueryCode;
use actix_web::HttpRequest;
use log::debug;

const LOGIN_COOKIE_NAME: &str = "login";

#[get("/api/cookie")]
async fn make_cookie() -> impl Responder {
    // We create a new cookie
    let cookie = Cookie::build("test_cookie", "value")
        .path("/")
        .max_age(ActixWebDuration::days(1))
        .http_only(true)
        .finish();

    // We print out the location of the cookie
    debug!("Cookie location: {:?}", LOCATION);

    // We return the cookie
    HttpResponse::Found().cookie(cookie).finish()
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

    HttpResponse::Found().cookie(cookie).finish()
}

#[get("/api/callback")]
async fn fake_callback(req: HttpRequest) -> impl Responder {
    let query = web::Query::<QueryCode>::from_query(req.query_string())
        .unwrap()
        .into_inner();

    let login_cookie = Cookie::build(LOGIN_COOKIE_NAME, query.code.clone())
        .path("/")
        .max_age(ActixWebDuration::days(1))
        .http_only(true)
        .finish();

    // We return a simple response
    HttpResponse::Found()
        .cookie(login_cookie)
        .append_header((LOCATION, query.state.clone()))
        .finish()
}

#[get("/api/logout")]
async fn fake_logout() -> impl Responder {
    // We create a new cookie
    let cookie = Cookie::build(LOGIN_COOKIE_NAME, "")
        .path("/")
        .max_age(ActixWebDuration::ZERO)
        .http_only(true)
        .finish();

    // // We return the cookie
    HttpResponse::Ok()
        .cookie(cookie)
        .finish()
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    // Start http server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4000")
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
            .service(fake_callback)
            .service(fake_logout)
            .service(hello)
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
