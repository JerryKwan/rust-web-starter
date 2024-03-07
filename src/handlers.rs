use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use tracing::{info, error, warn, debug, trace};

pub struct AppState {
    app_name: String,
}
pub async fn index(_data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}

#[derive(Deserialize, ToSchema)]
pub struct GetVINQueryParam{
    vin: String,
}
#[derive(Serialize, ToSchema)]
pub struct GetVINResponse {
    vin: String,
    records: Vec<String>,
}

#[utoipa::path(
    // get,
    // path="/get_vinrecords",
    responses(
        (status = 200, description="Get Records by VIN" , body = GetVINResponse)
    )
)]
#[get("/get_vinrecords")]
pub async fn get_vinrecords(input: web::Query<GetVINQueryParam>) -> Result<impl Responder> {
    debug!("get_vinrecords:{:?}", input.vin);
    let resp = GetVINResponse {
        vin: input.vin.clone(),
        records: vec!["record1".to_string(), "record2".to_string()],
    };
    Ok(web::Json(resp))
}