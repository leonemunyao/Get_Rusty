// Crop yield and harvest tracking system backend

use serde::{Deserialize, Serialize};
use candid::CandidType;
use ic_cdk_macros::update;
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug)]  // Add Serialize and Deserialize traits
pub struct Crop {
    id: u64,
    crop_type: String,
    variety: String,
    field_location: String,
    planting_date: String,
    weather_conditions: String,
    expected_yield: f64,
    actual_yield: f64,
    infection_monitoring: HashMap<String, String>,
    pest_and_disease_details: HashMap<String, String>,
    soil_quality: HashMap<String, String>,
    ph_level: f64,
    expected_rainfall: f64,
}

// Implement the Crop struct
impl Crop {
    pub fn new(
        id: u64,
        crop_type: String,
        variety: String,
        field_location: String,
        planting_date: String,
        weather_conditions: String,
        expected_yield: f64,
        actual_yield: f64,
        infection_monitoring: HashMap<String, String>,
        pest_and_disease_details: HashMap<String, String>,
        soil_quality: HashMap<String, String>,
        ph_level: f64,
        expected_rainfall: f64,
    ) -> Self {
        Crop {
            id,
            crop_type,
            variety,
            field_location,
            planting_date,
            weather_conditions,
            expected_yield,
            actual_yield,
            infection_monitoring,
            pest_and_disease_details,
            soil_quality,
            ph_level,
            expected_rainfall,
        }
    }
}

// Create a new Crop instance
#[update]
pub fn create_crop(
    id: u64,
    crop_type: String,
    variety: String,
    field_location: String,
    planting_date: String,
    weather_conditions: String,
    expected_yield: f64,
    actual_yield: f64,
    infection_monitoring: HashMap<String, String>,
    pest_and_disease_details: HashMap<String, String>,
    soil_quality: HashMap<String, String>,
    ph_level: f64,
    expected_rainfall: f64,
) -> Crop {
    Crop::new(
        id,
        crop_type,
        variety,
        field_location,
        planting_date,
        weather_conditions,
        expected_yield,
        actual_yield,
        infection_monitoring,
        pest_and_disease_details,
        soil_quality,
        ph_level,
        expected_rainfall,
    )
}

// Get the crop details
#[update]
pub fn get_crop_details(crop: Crop) -> Crop {
    crop
}

// Update the crop details

// Export the candid functions

ic_cdk::export_candid!(); 
