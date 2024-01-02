use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use super::{
    ext_routes,
    poi_routes::{self},
};

pub fn configure_routes(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/api").configure(api_router));
}

fn api_router(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/v1").configure(v1_router));
    conf.service(ping_me);
}

fn v1_router(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/ext").configure(ext_routes::v1_ext_router));
    conf.service(web::scope("/poi").configure(poi_routes::v1_poi_router));
}

#[get("/ping")]
async fn ping_me() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "success","message": "Zeus is alive"}))
}
