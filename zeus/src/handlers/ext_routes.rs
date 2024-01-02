use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder, get,
};
use serde_json::json;

use crate::{models::{
    app_dependency::AppDependency, completion_model::CompletionRequest,
    vision_request::VisionRequest, file_upload_request::UploadForm, embedding_body_request::EmbeddintBodyRequest,
}, api::google_places::GoogleGeocodeApiRequest, handlers::response_common};

pub fn v1_ext_router(conf: &mut web::ServiceConfig) {
    conf.service(completion_gpt)
        .service(completion_llama)
        .service(completion_gemini)
        .service(vision)
        .service(visuak_gpt)
        .service(visual_gemini)
        .service(embedding)
        .service(places_geocoding);
}

// Google Vision API

#[post("/vision")]
async fn vision(data: Data<AppDependency>, req: web::Json<VisionRequest>) -> impl Responder {
    let result = data.ext_api_usecase.vision(&req.base64).await;
    response_common::create_response(result)
}

// Google Places API

#[get("/geocoding")]
async fn places_geocoding(
    data: Data<AppDependency>,
    location: web::Query<GoogleGeocodeApiRequest>
) -> impl Responder {
    let result = data.ext_api_usecase.google_geocoding(location.0).await;
    response_common::create_response(result)
}

// OpenAI API

#[post("/text_gpt")]
async fn completion_gpt(
    data: Data<AppDependency>,
    req: web::Json<CompletionRequest>,
) -> impl Responder {
    let result = data.ext_api_usecase.gpt_completion(&req.query).await;
    response_common::create_response(result)
}

#[post("/visual_gpt")]
async fn visuak_gpt(
    data: Data<AppDependency>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    let f: TempFile = form.file;
    if f.size <= 0 {
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": "File size is 0"}));
    }

    let result =  data.ext_api_usecase.gpt_visual(f).await;
    response_common::create_response(result)
}

// Cloudflare API

#[post("/text_llama")]
async fn completion_llama(
    data: Data<AppDependency>,
    req: web::Json<CompletionRequest>,
) -> impl Responder {
    let result = data.ext_api_usecase.llama_completion(&req.query).await;
    response_common::create_response(result)
}

#[post("/embedding")]
async fn embedding(
    data: Data<AppDependency>,
    req: web::Json<EmbeddintBodyRequest>,
) -> impl Responder {
    let result = data.ext_api_usecase.embedding(&req).await;
    response_common::create_response(result)
}

// Gemini Google API

#[post("/text_gemini")]
async fn completion_gemini(
    data: Data<AppDependency>,
    req: web::Json<CompletionRequest>,
) -> impl Responder {
    let result = data.ext_api_usecase.gemini_completion(&req.query).await;
    response_common::create_response(result)
}

#[post("/visual_gemini")]
async fn visual_gemini(
    data: Data<AppDependency>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    let f: TempFile = form.file;
    if f.size <= 0 {
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": "File size is 0"}));
    }
    let result = data.ext_api_usecase.gemini_visual(f).await;
    response_common::create_response(result)
}