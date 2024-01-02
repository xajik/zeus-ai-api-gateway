use actix_cors::Cors;
use actix_web::{dev::Server, http::header, middleware::Logger, web, App, HttpServer};

use crate::{handlers, models::app_dependency::AppDependency};

pub struct ZeusServer;

impl ZeusServer {
    pub fn new(add_dependency: AppDependency) -> Server {
        let port = 3001;
        let host = "0.0.0.0";
        let app_data = web::Data::new(add_dependency);
        let methods = vec!["GET", "POST", "PATCH", "DELETE"];
        let ptorocol = match port {
            443 => "https",
            _ => "http",
        };

        HttpServer::new(move || {
            let cors = Cors::default()
                .allowed_origin(format!("{}://{}:{}", ptorocol, host, port).as_str())
                .allowed_methods(methods.clone())
                .allowed_headers(vec![
                    header::CONTENT_TYPE,
                    header::AUTHORIZATION,
                    header::ACCEPT,
                ])
                .supports_credentials();
            App::new()
                .app_data(app_data.clone())
                .configure(handlers::routes::configure_routes)
                .wrap(cors)
                .wrap(Logger::default())
        })
        .bind((host, port))
        .unwrap()
        .run()
    }
}
