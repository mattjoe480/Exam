use actix_web::{App, guard, HttpResponse, HttpServer, Responder, web};
use actix_web::error::HttpError;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::GraphQL;
use crate::models::model::{QueryRoot, StarWars};


mod models;

async fn greet() -> impl Responder {
    "Hello from Rust!".to_string()
}

async fn index_graphiql() -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/api/graphql").finish()))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    HttpServer::new(move || {
        let schema = Schema::build(QueryRoot,EmptyMutation, EmptySubscription)
            .data(StarWars::new())
            .finish();


        App::new()
            .service(
                web::resource("/api/graphql")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema)),
            )
            .service(web::resource("/api/graphiql")
                .guard(guard::Get()).to(index_graphiql))
            .route("/api/v1/", web::get().to(greet)) 
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}