#[macro_use]
extern crate serde;
extern crate ic_cdk_macros;
extern crate ic_cdk;
//use candid::{Decode, Encode};
//use ic_cdk::api::time; 
// use ic_stable_structures::memory_manager::{VirtualMemory};
use std::collections::HashMap;
// use ic_stable_structures::{Cell, DefaultMemoryImpl};
//use std::{cell::RefCell};

// type Memory<DefaultMemoryImpl> = VirtualMemory<DefaultMemoryImpl>;
// type IdCell = Cell<u64, Memory<DefaultMemoryImpl>>;


// Define the livestock struct 
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
#[derive(Debug)]
struct Livestock {
    id: u64,
    breed: String,
    age: u8,
    height: f32,
    healthrecords: String,
    created_at: u64,
    updated_at: Option<u64>,
}


// Using HashMap to store animal records where each animal has a unique ID
struct LivestockManagementSystem {
    animal: HashMap<u32, Livestock>,   // Strores animals by their id
    next_id: u64,   // This is a counter to generate unique IDs
}


impl LivestockManagementSystem {

    // function to initiliaze new management system
    fn new() -> Self { LivestockManagementSystem {
        animal: HashMap::new(),
        next_id: 1,
    }}

    // create_animal function
    fn create_animal(&mut self, age: u8, breed: String, height: f32, healthrecords: String) -> u64 {


        // create new animal with unique ID
        let animal = Livestock {
            id: self.next_id,
            age,
            breed,
            height,
            healthrecords,
            created_at: 0,
            updated_at: None,
        };

        // Insert animal into the HashMap
        self.animal.insert(self.next_id.try_into().unwrap(), animal);

        // Increment ID to the next animal
        self.next_id += 1;

        // Return the ID of the new animal for reference
        self.next_id - 1
    }
}

// Creating a mutable static instance of LivestockManagementSystem
static mut LIVECTOCK_SYSTEM: Option<LivestockManagementSystem> = None;


// Initialize the canister state
#[ic_cdk_macros::init]
fn init() {
    ic_cdk::println!("Initializing Livestock Management System...");
    unsafe {
        LIVECTOCK_SYSTEM = Some(LivestockManagementSystem::new());
        ic_cdk::println!("Livestock Management System Initialized.");
    }
}


#[ic_cdk_macros::update]
fn create_animal(age: u8, breed: String, height: f32, healthrecords: String) -> u64 {
    ic_cdk::println!("Creating animal with age: {}, breed: {}, height: {}, healthrecords: {}", age, breed, height, healthrecords);
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_mut().expect("System not Initialized.");
        let id = system.create_animal(age, breed, height, healthrecords);
        ic_cdk::println!("Animal created with ID: {}", id); 
        id
    }
}

// Read function to get the animal details by ID
#[ic_cdk_macros::query]
fn get_animal(id: u64) -> Option<Livestock> {
    ic_cdk::println!("Getting animal with ID: {}", id);
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_ref().expect("System not Initialized.");
        match system.animal.get(&(id as u32)) {
            Some(animal) => {
                ic_cdk::println!("Animal found: {:?}", animal);
                Some(animal.clone())
            }
            None => {
                ic_cdk::println!("No animal found with ID: {}", id);
                None
            }
        }
    }
}


// Update function to update the animal details by ID
#[ic_cdk_macros::update]
fn update_animal(id: u64, age: u8, breed: String, height: f32, healthrecords: String) -> bool {
    ic_cdk::println!("Updating animal with ID: {}", id);
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_mut().expect("System not Initialized.");
        match system.animal.get_mut(&(id as u32)) {
            Some(animal) => {
                animal.age = age;
                animal.breed = breed;
                animal.height = height;
                animal.healthrecords = healthrecords;
                animal.updated_at = Some(0);
                ic_cdk::println!("Animal updated: {:?}", animal);
                true
            }
            None => {
                ic_cdk::println!("No animal found with ID: {}", id);
                false
            }
        }
    }
}

// Delete function to delete the animal by ID
#[ic_cdk_macros::update]
fn delete_animal(id: u64) -> bool {
    ic_cdk::println!("Deleting animal with ID: {}", id);
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_mut().expect("System not Initialized.");
        match system.animal.remove(&(id as u32)) {
            Some(_) => {
                ic_cdk::println!("Animal deleted with ID: {}", id);
                true
            }
            None => {
                ic_cdk::println!("No animal found with ID: {}", id);
                false
            }
        }
    }
}





// Export the candid functions

ic_cdk::export_candid!(); 


