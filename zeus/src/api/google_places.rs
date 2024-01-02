use std::collections::HashSet;

use crate::{models::custom_error::CustomError, repository::secrets::Secrets};
use serde::{Deserialize, Serialize};

pub struct GooglePlacesApi {
    client: reqwest::Client,
    key: (String, String),
}

impl GooglePlacesApi {
    const API_URL: &'static str = "https://maps.googleapis.com/maps/api/geocode/json";
    const LATP_LNG: &'static str = "latlng";

    pub fn new(secrets: &Secrets) -> Self {
        let client = reqwest::Client::builder()
            .build()
            .expect("Failed to build reqwest client");
        let key = ("key".to_string(), secrets.google_vision_api_key.clone());
        Self { client, key }
    }

    pub async fn geocoding(
        &self,
        location: GoogleGeocodeApiRequest,
    ) -> Result<GoogleGeocodeApiResponse, CustomError> {
        let latlng = format!("{},{}", location.lat, location.lng);
        let res = self
            .client
            .get(Self::API_URL)
            .query(&[self.key.clone(), (Self::LATP_LNG.to_string(), latlng)])
            .send()
            .await?;

        if res.status().is_success() {
            let response: GoogleGeocodeApiResponse = res.json().await?;
            Ok(response)
        } else {
            let code = res.status().as_u16();
            if let Ok(text) = res.text().await {
                log::error!("Error response: {}", text);
            }
            Err(CustomError::NonSuccessfulResponse(code))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GoogleGeocodeApiRequest {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleGeocodeApiResponse {
    address_descriptor: Option<AddressDescriptor>,
    plus_code: Option<PlusCode>,
    results: Vec<ResultItem>,
    status: String,
}

impl GoogleGeocodeApiResponse {
    pub fn unique_addresses(&self) -> Vec<String> {
        let mut unique_addresses = HashSet::new();
        self.results
            .iter()
            .filter_map(|r| {
                if unique_addresses.insert(&r.formatted_address) {
                    Some(r.formatted_address.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AddressDescriptor {
    areas: Vec<Area>,
    landmarks: Vec<Landmark>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Area {
    containment: String,
    display_name: DisplayName,
    place_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Landmark {
    display_name: DisplayName,
    place_id: String,
    spatial_relationship: String,
    straight_line_distance_meters: f64,
    travel_distance_meters: f64,
    types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DisplayName {
    language_code: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlusCode {
    compound_code: String,
    global_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultItem {
    address_components: Vec<AddressComponent>,
    formatted_address: String,
    geometry: Geometry,
    place_id: String,
    types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddressComponent {
    long_name: String,
    short_name: String,
    types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Geometry {
    // Define fields as per your requirements
    location: Location,
    location_type: String,
    // Add bounds, viewport, etc. if needed
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    lat: f64,
    lng: f64,
}
