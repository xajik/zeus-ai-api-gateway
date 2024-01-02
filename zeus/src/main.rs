use std::sync::Arc;

use dotenv::dotenv;
use models::{app_dependency::AppDependency, custom_error::CustomError};

use crate::repository::{
    key_value_repository::KeyValueRepository, key_value_vector_repository::KeyValueVectorRepository,
};

mod api;
mod db;
mod handlers;
mod models;
mod repository;
mod server;
mod usecase;
mod utils;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    println!("Starting Zeus!");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }

    //Docker environemnt variables
    dotenv::from_filename("../.env").ok();
    //Service environment variables
    dotenv().ok();

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    let app_dependency = di().await?;

    let server = server::server::ZeusServer::new(app_dependency);

    server.await.map_err(|e| CustomError::IoError(e))
}

async fn di() -> Result<AppDependency, CustomError> {
    //Secrets
    let secrets = repository::secrets::Secrets::new();
    //Database
    let pool = Arc::new(db::database_pool::DatabasePool::new(&secrets).await?);
    let vector_store = KeyValueVectorRepository::new(Arc::clone(&pool));
    let key_value_store = KeyValueRepository::new(Arc::clone(&pool));
    //APIs
    let open_ai_api = Arc::new(api::open_ai::OpenAIApi::new(&secrets));
    let google_vision_api = Arc::new(api::google_vision::GoogleVisionApi::new(&secrets));
    let cloudflare_ai = Arc::new(api::cloudflare_ai::CloudflareApi::new(&secrets));
    let gemini_api = Arc::new(api::google_gemini::GeminiApi::new(&secrets));
    let google_places = Arc::new(api::google_places::GooglePlacesApi::new(&secrets));
    //Storage
    let local_storage = Arc::new(repository::local_storage::LocalStorage::new());
    //Usecases
    let openai_usecase = usecase::api_tester_usecase::ExtApiUsecase::new(
        Arc::clone(&open_ai_api),
        Arc::clone(&google_vision_api),
        Arc::clone(&cloudflare_ai),
        Arc::clone(&gemini_api),
        Arc::clone(&google_places),
        Arc::clone(&local_storage),
    );

    let poi_usecase = usecase::poi_usecase::PoiUsecase::new(
        Arc::clone(&google_vision_api),
        Arc::clone(&gemini_api),
        Arc::clone(&open_ai_api),
        Arc::clone(&google_places),
    );

    Ok(models::app_dependency::AppDependency::new(
        openai_usecase,
        poi_usecase,
    ))
}
