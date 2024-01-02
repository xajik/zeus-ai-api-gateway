use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{web::{self, Data}, post, Responder, HttpResponse};
use serde_json::json;

use crate::{models::{app_dependency::AppDependency, file_upload_request::UploadForm}, api::google_places::GoogleGeocodeApiRequest, handlers::response_common};

pub fn v1_poi_router(conf: &mut web::ServiceConfig) {
    conf.service(poi);
}

#[post("/from_image")]
async fn poi(
    data: Data<AppDependency>,
    location: web::Query<GoogleGeocodeApiRequest>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    let f: TempFile = form.file;
    if f.size <= 0 {
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": "File size is 0"}));
    }
    let result = data.poi_usecase.from_image(location.0, f).await;
    response_common::create_response(result)
}
