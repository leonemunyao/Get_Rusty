#[macro_use]
extern crate serde;
extern crate ic_cdk_macros;
extern crate ic_cdk;
use ic_cdk::api::time;
use std::collections::HashMap;


// Define the livestock struct 
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
#[derive(Debug)]
struct Livestock {
    id: u64,
    breed: String,
    age: u8,
    height: f32,
    healthrecords: String,
    healthstatus: HealthStatus,
    medical_records: Vec<Medication>,
    parent_ids: Option<ParentIds>,
    created_at: u64,
    updated_at: Option<u64>,
}

// Vacination and medication trucking
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Debug)]
struct Medication {
    id: u64,
    name: String,
    dosage: String,
    start_date: u64,
    end_date: u64,
}

// Parent IDs struct of the animal
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Debug)]
struct ParentIds {
    parent1_id: u64,
    parent2_id: u64,
}

// Health alert struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Debug)]
struct HealthAlert {
    animal_id: u64,
    status: HealthStatus,
    timestamp: u64,
}

// Event logging struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Debug)]
struct EventLog {
    event_type: String,
    details: String,
    timestamp: u64,
}

// Health status struct of the animal
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Copy, Default)]
#[derive(Debug)]
#[derive(PartialEq)]
enum HealthStatus {
    #[default]  // Default status is Healthy
    Healthy,
    Sick,
    Critical,
    Recovering,
}


// Using HashMap to store animal records where each animal has a unique ID
struct LivestockManagementSystem {
    animal: HashMap<u32, Livestock>,   // Strores animals by their id
    next_id: u64,   // This is a counter to generate unique IDs
    health_alerts: Vec<HealthAlert>,  // Stores health alerts
    event_logs: Vec<EventLog>,  // Stores event logs
}


impl LivestockManagementSystem {

    // function to initiliaze new management system
    fn new() -> Self { LivestockManagementSystem {
        animal: HashMap::new(),
        next_id: 1,
        health_alerts: Vec::new(),
        event_logs: Vec::new(),
    }}

    // create_animal function
    fn create_animal(&mut self, age: u8, breed: String, height: f32) -> u64 {

        let current_time = time();
        let breed_clone = breed.clone();

        // create new animal with unique ID
        let animal = Livestock {
            id: self.next_id,
            age,
            breed,
            height,
            healthrecords: "Healthy".to_string(),
            healthstatus: HealthStatus::Healthy,
            medical_records: Vec::new(),
            parent_ids: None,
            created_at: current_time,
            updated_at: None,
        };

        // Insert animal into the HashMap
        self.animal.insert(self.next_id.try_into().unwrap(), animal);

        // log the event
        self.event_logs.push(EventLog {
            event_type: "Animal Created".to_string(),
            details: format!("Animal with ID: {}, Breed: {}, Age: {}, Height: {} created.", self.next_id, breed_clone, age, height),
            timestamp: current_time,
        });

        // Increment ID to the next animal
        self.next_id += 1;

        // Return the ID of the new animal for reference
        self.next_id - 1
    }

    // Bread animal function
    fn breed_animals(&mut self, parent1_id: u64, parent2_id: u64, breed: String) -> Option<u64> {

        // check if both parents exist
        if self.animal.contains_key(&(parent1_id as u32)) && self.animal.contains_key(&(parent2_id as u32)) {
            let breed_clone = breed.clone();

            // Create a new offspring
            let offspring_id = self.create_animal(0, breed, 0.0);

            // Set the parents IDs of the offspring
            if let Some(offspring) = self.animal.get_mut(&(offspring_id as u32)) {
                offspring.parent_ids = Some(ParentIds {
                    parent1_id,
                    parent2_id,
                });
            }

            // log the event
            self.event_logs.push(EventLog {
                event_type: "Animal Bred".to_string(),
                details: format!("Animal with ID: {} and ID: {} bred to create a new animal with ID: {} and breed: {}", parent1_id, parent2_id, offspring_id, breed_clone),
                timestamp: time(),
            });

            Some(offspring_id)
        } else {
            None
        }
    }

    // Get Peddigree function
    fn get_pedigree(&self, id: u64) -> Vec<Livestock> {
        let mut pedigree = Vec::new();
        if let Some(animal) = self.animal.get(&(id as u32)) {
            pedigree.push(animal.clone());
            if let Some(parents) = &animal.parent_ids {
                pedigree.append(&mut self.get_pedigree(parents.parent1_id));
                pedigree.append(&mut self.get_pedigree(parents.parent2_id));
            }
        }
        pedigree
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
fn create_animal(age: u8, breed: String, height: f32) -> u64 {
    ic_cdk::println!("Creating animal with age: {}, breed: {}, height: {}", age, breed, height);
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_mut().expect("System not Initialized.");
        let id = system.create_animal(age, breed, height);
        ic_cdk::println!("Animal created with ID: {}", id); 
        id
    }
}

// Breed function to create a new animal by breeding two existing animals
#[ic_cdk_macros::update]
fn breed_animals(parent1_id: u64, parent2_id: u64, breed: String) -> Option<u64> {
    ic_cdk::println!("Breeding animals with parent IDs: {} and {} to create a new animal with breed: {}", parent1_id, parent2_id, breed);
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_mut().expect("System not Initialized.");
        system.breed_animals(parent1_id, parent2_id, breed)
    }
}

// Get Pedigree function to get the pedigree of an animal by ID
#[ic_cdk_macros::query]
fn get_pedigree(id: u64) -> Vec<Livestock> {
    ic_cdk::println!("Getting pedigree of animal with ID: {}", id);
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_ref().expect("System not Initialized.");
        system.get_pedigree(id)
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

// Get all animals function to get all the animals in the system
#[ic_cdk_macros::query]
fn get_all_animals() -> Vec<Livestock> {
    ic_cdk::println!("Getting all animals...");
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_ref().expect("System not Initialized.");
        system.animal.values().cloned().collect()
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
                let breed_clone = breed.clone();
                animal.age = age;
                animal.breed = breed;
                animal.height = height;
                animal.healthrecords = healthrecords;
                animal.updated_at = Some(0);

                // log the event
                system.event_logs.push(EventLog {
                    event_type: "Animal Updated".to_string(),
                    details: format!("Animal with ID: {}, Breed: {}, Age: {}, Height: {} updated.", id, breed_clone, age, height),
                    timestamp: time(),
                });

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


// Function to update the animal health status
#[ic_cdk_macros::update]
fn update_health_status(id: u64, new_status: HealthStatus) -> bool {

    ic_cdk::println!("Updating health status of animal with ID: {} to {:?}", id, new_status);
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_mut().expect("System not Initialized.");
        if let Some(animal) = system.animal.get_mut(&(id as u32)) {
            animal.healthstatus = new_status;
            animal.healthrecords = format!("{:?}", new_status);
            animal.updated_at = Some(time());

            // Check if the new status is Critical or Sick and create an alert
            if matches!(new_status, HealthStatus::Critical | HealthStatus::Sick | HealthStatus::Recovering) {
                let alert = HealthAlert {
                    animal_id: id,
                    status: new_status,
                    timestamp: time(),
                };
                system.health_alerts.push(alert);
                ic_cdk::println!("ALERT: Animal with ID: {} is now {:?}", id, new_status);
            }
            true
        } else {
            ic_cdk::println!("No animal found with ID: {}", id);
            false
        }
    }
}


// ToString implementation for HealthStatus
impl ToString for HealthStatus {
    fn to_string(&self) -> String {
        match self {
            HealthStatus::Healthy => "Healthy".to_string(),
            HealthStatus::Sick => "Sick".to_string(),
            HealthStatus::Critical => "Critical".to_string(),
            HealthStatus::Recovering => "Recovering".to_string(),
        }
    }
    
}


// Function to get the health alerts
#[ic_cdk_macros::query]
fn get_health_alerts() -> Vec<HealthAlert> {
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_ref().expect("System not Initialized.");
        system.health_alerts.clone()
    }
}

// Medication tracking function
#[ic_cdk_macros::update]
fn track_medication(animal_id: u64, medication_name: String, dosage: String) -> bool {
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_mut().expect("System not Initialized.");
        if let Some(animal) = system.animal.get_mut(&(animal_id as u32)) {
            let medication = Medication {
                id: animal.medical_records.len() as u64 + 1,
                name: medication_name,
                dosage,
                start_date: time(),
                end_date: time() + 86400, // 1 day
            };
            animal.medical_records.push(medication);
            ic_cdk::println!("Medication tracked for animal with ID: {}", animal_id);
            true
        } else {
            false
            
        }
    }
}

// A function to retrieve all the animals whose Heals status is Critical
#[ic_cdk_macros::query]
fn get_critical_animals() -> Vec<Livestock> {
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_ref().expect("System not Initialized.");
        system.animal.values().filter(|animal| animal.healthstatus == HealthStatus::Critical).cloned().collect()
    }
}

// A function to retrieve all the animals whose Heals status is Sick
#[ic_cdk_macros::query]
fn get_sick_animals() -> Vec<Livestock> {
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_ref().expect("System not Initialized.");
        system.animal.values().filter(|animal| animal.healthstatus == HealthStatus::Sick).cloned().collect()
    }
}

// A function to retrieve all the animals whose Heals status is Recovering
#[ic_cdk_macros::query]
fn get_recovering_animals() -> Vec<Livestock> {
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_ref().expect("System not Initialized.");
        system.animal.values().filter(|animal| animal.healthstatus == HealthStatus::Recovering).cloned().collect()
    }
}

// A function to retrieve all the animals whose Heals status is Healthy
#[ic_cdk_macros::query]
fn get_healthy_animals() -> Vec<Livestock> {
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_ref().expect("System not Initialized.");
        system.animal.values().filter(|animal| animal.healthstatus == HealthStatus::Healthy).cloned().collect()
    }
}

// Query function to get all the event logs
#[ic_cdk_macros::query]
fn get_event_logs() -> Vec<EventLog> {
    ic_cdk::println!("Getting all event logs...");
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_ref().expect("System not Initialized.");
        system.event_logs.clone()
    }
}


// Event logging function to track all changes made to the system like animal creation, deletion, updating etc.

// Delete function to delete the animal by ID
#[ic_cdk_macros::update]
fn delete_animal(id: u64) -> bool {
    ic_cdk::println!("Deleting animal with ID: {}", id);
    unsafe {
        let system = LIVECTOCK_SYSTEM.as_mut().expect("System not Initialized.");
        match system.animal.remove(&(id as u32)) {
            Some(animal) => {
                ic_cdk::println!("Animal deleted: {:?}", animal);
                system.event_logs.push(EventLog {
                    event_type: "Animal Deleted".to_string(),
                    details: format!("Animal with ID: {} deleted.", id),
                    timestamp: time(),
                });

                system.animal.remove(&(id as u32));
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


