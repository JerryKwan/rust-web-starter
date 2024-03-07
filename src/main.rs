use tracing::{info, error, warn, debug, trace};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod logger;
use logger::setup_logging_and_tracing;
mod handlers;
mod config;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::get_vinrecords,
    ),
    components(
        schemas(handlers::GetVINQueryParam, handlers::GetVINResponse)
    ),
    tags(
        (name = "vinrecords", description = "Operations about vinrecords")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // setup logger
    let _subscriber = setup_logging_and_tracing("log/app.log");
    // load config
    let sys_config = config::Config::load("./config.json").unwrap();
    debug!("config: {:?}", sys_config);

    let openapi = ApiDoc::openapi();
    // start server
    HttpServer::new(move || {
        App::new()
            // .route("/", web::get().to(handlers::index))
            // .route("/get_vinrecords", web::get().to(handlers::get_vinrecords))
            .service(handlers::get_vinrecords)
            .service(
                SwaggerUi::new("swagger-ui/{_:.*}")
                .url("/api-docs/openai.json", openapi.clone()),
            )
    })
    .bind((sys_config.host, sys_config.port))?
    .run()
    .await
}
