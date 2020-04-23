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
#[macro_use]
extern crate lazy_static;

use crate::serde::ser::Error as SerdeError;
use actix_cors::Cors;
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::StreamExt;
use listenfd::ListenFd;

use juniper::http::{GraphQLResponse};
use juniper_actix::{graphiql_handler, playground_handler, graphql_handler};

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
use crate::errors::ServiceError;
use crate::utils::env::ENV;
use actix_web::web::BytesMut;
use graphql_depth_limit::QueryDepthAnalyzer;
use juniper::IntoFieldError;
use std::fmt::Debug;

pub async fn graphql_interface(_req: HttpRequest) -> Result<HttpResponse, Error> {
    graphiql_handler("/", None).await
}

pub async fn graphql_playground(_req: HttpRequest) -> Result<HttpResponse, Error> {
    playground_handler("/", None).await
}

#[derive(Deserialize, Clone, Serialize, PartialEq, Debug)]
struct DataWithQuery {
    query: String,
}

#[derive(Debug, serde_derive::Deserialize, PartialEq)]
#[serde(untagged)]
enum BatchDataWithQuery {
    Single(DataWithQuery),
    Batch(Vec<DataWithQuery>)
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
    }
    Ok(body)
}

fn analyze_query_errors(query: &str) -> Option<Result<HttpResponse, Error>> {
    let depth_limit_analyzer = QueryDepthAnalyzer::new(query, vec![], |_a, _b| true);
    if let Ok(depth_limit_analyzer) = depth_limit_analyzer {
        match depth_limit_analyzer.verify(4) {
            Ok(_depth) => {}
            Err(err) => {
                let res =
                    GraphQLResponse::error(ServiceError::MaxDepthLimit(err).into_field_error());
                let graphql_response = serde_json::to_string(&res);
                if let Ok(graphql_response) = graphql_response {
                    return Some(Ok(HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(graphql_response)));
                }
            }
        };
    }
    return None;
}

fn analyze_batch_query_errors(data_with_query: BatchDataWithQuery) -> Option<Result<HttpResponse, Error>>{
    match data_with_query {
        BatchDataWithQuery::Single(data_with_query) => {
            return analyze_query_errors(&data_with_query.query)
        },
        BatchDataWithQuery::Batch(data_with_queries) => {
            for data_with_query in data_with_queries {
                let query_error = analyze_query_errors(&data_with_query.query);
                if query_error.is_some() {
                    return query_error;
                }
            }
        }
    };
    None
}

async fn graphql(
    st: web::Data<Schema>,
    user: LoggedUser,
    pool: web::Data<MysqlPool>,
    payload: web::Payload,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let body = get_data(payload).await?;
    let data_with_query = serde_json::from_slice::<BatchDataWithQuery>(&body)?;
    if let Some(result) = analyze_batch_query_errors(data_with_query) {
        return result;
    }
    let mysql_pool = pool.get().map_err(|e| serde_json::Error::custom(e))?;
    let ctx = create_context(user.email, mysql_pool);
    let gql_payload = web::Payload(actix_web::dev::Payload::Stream(Box::pin(tokio::stream::once(Ok(body.freeze())))));
    graphql_handler(&st, &ctx, req, gql_payload).await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    //    let sys = actix::System::new("selloclub");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(establish_connection())
            .data(create_schema())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    //                    .allowed_origin(SERVER_URL)
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(
                web::resource("/")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/graphiql").route(web::get().to(graphql_interface)))
            .service(web::resource("/playground").route(web::get().to(graphql_playground)))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(format!("0.0.0.0:{}", ENV.server_port)).unwrap()
    };

    println!("Started http server: 0.0.0.0:{}", ENV.server_port);
    server.run().await
}
