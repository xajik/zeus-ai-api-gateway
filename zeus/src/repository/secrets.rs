pub struct Secrets {
    pub open_ai_api_key: String,
    pub cloudflare_api_key: String,
    pub cloudflare_account: String,
    pub google_vision_api_key: String,
    pub google_ai_studio_api_key: String,
    pub rds_hostname: String,
    pub rds_port: i32,
    pub rds_db_name: String,
    pub rds_username: String,
    pub rds_password: String,
}

impl Secrets {
    pub fn new() -> Self {
        let open_ai_api_key =
            std::env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set.");
        let cloudflare_api_key =
            std::env::var("CLOUDFLARE_AI_API_KEY").expect("CLOUDFLARE_AI_API_KEY must be set.");
        let google_vision_api_key =
            std::env::var("GOOGLE_VISION_API_KEY").expect("GOOGLE_VISION_API_KEY must be set.");
        let cloudflare_account =
            std::env::var("CLOUDFLARE_ACCOUNT").expect("CLOUDFLARE_ACCOUNT must be set.");
        let google_ai_studio_api_key =
            std::env::var("GOOGLE_PALM2_API_KEY").expect("GOOGLE_PALM2_API_KEY must be set.");
        let rds_hostname = std::env::var("RDS_HOSTNAME").expect("RDS_HOSTNAME must be set.");
        let rds_port = std::env::var("RDS_PORT")
            .expect("RDS_PORT must be set.")
            .parse::<i32>()
            .unwrap();
        let rds_db_name = std::env::var("RDS_DB_NAME").expect("RDS_DB_NAME must be set.");
        let rds_username = std::env::var("RDS_USERNAME").expect("RDS_USERNAME must be set.");
        let rds_password = std::env::var("RDS_PASSWORD").expect("RDS_PASSWORD must be set.");
        Self {
            open_ai_api_key,
            cloudflare_api_key,
            google_vision_api_key,
            cloudflare_account,
            google_ai_studio_api_key,
            rds_hostname,
            rds_port,
            rds_db_name,
            rds_username,
            rds_password,
        }
    }
}
