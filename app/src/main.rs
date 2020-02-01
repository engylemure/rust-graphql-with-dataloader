#[macro_use]
extern crate diesel;
extern crate juniper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;
extern crate validator;

use crate::serde::ser::Error as SerdeError;
use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use listenfd::ListenFd;
use std::sync::Arc;

use dotenv::dotenv;
use juniper::http::graphiql::graphiql_source;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;

mod db;
mod errors;
mod graphql;
mod handlers;
mod models;
mod schema;
mod utils;

use crate::db::{establish_connection, MysqlPool};
use crate::graphql::{create_context, create_schema, Schema};
use crate::handlers::LoggedUser;
//use std::fmt::format;
use std::env;

const SERVER_URL: &str = "http://172.17.0.3:80";

pub async fn graphql_interface(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let html = graphiql_source(format!("{}/graphql", SERVER_URL).as_str());
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn graphql_playground(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let html = playground_source(format!("{}/graphql", SERVER_URL).as_str());
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

#[derive(Deserialize, Clone, Serialize, PartialEq, Debug)]
struct DataWithQuery {
    query: String
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    user: LoggedUser,
    pool: web::Data<MysqlPool>
) -> Result<HttpResponse, Error> {
//    println!("{}", data_with_query.query);
    let graphql_response = {
        let mysql_pool = pool.get().map_err(|e| serde_json::Error::custom(e))?;
        let ctx = create_context(user.email, mysql_pool);
        let res = data.execute_async(&st, &ctx).await;
        serde_json::to_string(&res)
    }?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(graphql_response))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Starting server");
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    //    let sys = actix::System::new("selloclub");
    let schema = std::sync::Arc::new(create_schema());
    let mut listenfd = ListenFd::from_env();
    let port: i16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| String::from("80"))
        .parse()
        .expect("SERVER_PORT must be a number");
    let mut server = HttpServer::new(move || {
        App::new()
            .data(establish_connection())
            .data(schema.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
//                    .allowed_origin(SERVER_URL)
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600).
                    finish(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphql_interface)))
            .service(web::resource("/graphql_playground").route(web::get().to(graphql_playground)))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(format!("0.0.0.0:{}", port)).unwrap()
    };

    println!("Started http server: 0.0.0.0:{}", port);
    server.run().await
}
