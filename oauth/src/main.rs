//! Oauth mockup to emulate requests to a real oauth server
extern crate serde_derive;
extern crate serde_qs;

use actix_web::http::header::LOCATION;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{get, web, App, HttpServer};
use commons::{OauthData, QueryCode};

#[get("/oauth/authorize")]
async fn fake_auth(req: HttpRequest) -> impl Responder {
    let params = web::Query::<OauthData>::from_query(req.query_string())
        .unwrap()
        .into_inner();

    // We make a request to the server we know curresponds to the provided client_id
    match params.client_id.as_str() {
        "our_muckup_server" => {
            // We make a request to the server we know curresponds to the provided client_id
            let server = "http://localhost:9090/api/callback";

            let query = QueryCode {
                code: "success".to_string(),
                state: params.state.clone(),
            };

            // We create a temporary redirect response to the server
            // we know corresponds to the provided client_id, with the
            // code and state as query parameters
            HttpResponse::Found()
                .append_header((
                    LOCATION,
                    format!("{}?{}", server, serde_qs::to_string(&query).unwrap()),
                ))
                .finish()
        }
        _ => HttpResponse::Unauthorized().finish(),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            // We register the API handlers
            .service(fake_auth)
    })
    .bind("127.0.0.1:9999")?
    .workers(4)
    .run()
    .await
}
