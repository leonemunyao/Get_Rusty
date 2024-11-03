// Crop yield and harvest tracking system backend

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]  // Add Serialize and Deserialize traits
struct Crop {
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
    fn new(
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
fn create_crop(
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

// Function to display the crop details
fn main() {
    let new_crop = create_crop();
    println!("{:#?}", new_crop);
}


// Export the candid functions

ic_cdk::export_candid!(); 