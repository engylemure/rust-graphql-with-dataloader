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
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, error};
use listenfd::ListenFd;
use std::sync::Arc;
use futures::StreamExt;

use dotenv::dotenv;
use juniper::http::graphiql::graphiql_source;
use juniper::http::playground::playground_source;
use juniper::http::{GraphQLRequest, GraphQLResponse};

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
use actix_web::web::BytesMut;
use std::fmt::Debug;
use graphql_depth_limit::{QueryDepthAnalyzer, DepthLimitError};
use juniper::IntoFieldError;
use crate::errors::ServiceError;

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

const MAX_SIZE: usize = 262_144;

async fn get_data(mut payload: web::Payload) -> Result<BytesMut, Error> {
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    };
    Ok(body)
}

fn analyze_query_errors(query: &str) -> Option<Result<HttpResponse, Error>> {
    let depth_limit_analyzer = QueryDepthAnalyzer::new(query, vec![], |_a, _b| true);
    if let Ok(depth_limit_analyzer) = depth_limit_analyzer {
        match depth_limit_analyzer.verify(7) {
            Ok(_depth) => {}
            Err(err) => {
                if let DepthLimitError::Exceed(exceed_err) = err {
                    let res = GraphQLResponse::error(ServiceError::MaxDepthLimit(exceed_err).into_field_error());
                    let graphql_response = serde_json::to_string(&res);
                    if let Ok(graphql_response) = graphql_response {
                        return Some(Ok(HttpResponse::Ok()
                            .content_type("application/json")
                            .body(graphql_response)))
                    }
                }
            }
        };
    }
    return None
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    user: LoggedUser,
    pool: web::Data<MysqlPool>,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let body = get_data(payload).await?;
    let data = serde_json::from_slice::<GraphQLRequest>(&body)?;
    let data_with_query = serde_json::from_slice::<DataWithQuery>(&body)?;
    let query_error = analyze_query_errors(&data_with_query.query);
    if let Some(error_found) = query_error {
        return error_found;
    }
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
