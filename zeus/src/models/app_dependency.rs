use crate::usecase::{api_tester_usecase, poi_usecase};

pub struct AppDependency {
    pub ext_api_usecase: api_tester_usecase::ExtApiUsecase,
    pub poi_usecase: poi_usecase::PoiUsecase,
}

impl AppDependency {

    pub fn new(
        ext_api_usecase: api_tester_usecase::ExtApiUsecase,
        poi_usecase: poi_usecase::PoiUsecase,
    ) -> AppDependency {
        Self {
            ext_api_usecase,
            poi_usecase,
        }
    }
}
