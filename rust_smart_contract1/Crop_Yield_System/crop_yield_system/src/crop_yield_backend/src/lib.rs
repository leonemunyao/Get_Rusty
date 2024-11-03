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

// Function to display the crop details
// pub fn main() {
//     // Placeholder values for the crop details
//     let id = 1;
//     let crop_type = String::new();
//     let variety = String::new();
//     let field_location = String::new();
//     let planting_date = String::new();
//     let weather_conditions = String::new();
//     let expected_yield = 0.0;
//     let actual_yield = 0.0;
//     let infection_monitoring = HashMap::new();
//     let pest_and_disease_details = HashMap::new();
//     let soil_quality = HashMap::new();
//     let ph_level = 0.0;
//     let expected_rainfall = 0.0;

//     let new_crop = create_crop(
//         id,
//         crop_type,
//         variety,
//         field_location,
//         planting_date,
//         weather_conditions,
//         expected_yield,
//         actual_yield,
//         infection_monitoring,
//         pest_and_disease_details,
//         soil_quality,
//         ph_level,
//         expected_rainfall
//     );
//     println!("{:#?}", new_crop);
// }


// Export the candid functions

ic_cdk::export_candid!(); 
