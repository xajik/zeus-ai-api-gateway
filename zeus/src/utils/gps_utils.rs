pub struct GpsUtils {}

impl GpsUtils {
    pub fn is_valid_coordinate(latitude: f64, longitude: f64) -> bool {
        (latitude >= -90.0 && latitude <= 90.0) && (longitude >= -180.0 && longitude <= 180.0)
    }
}
