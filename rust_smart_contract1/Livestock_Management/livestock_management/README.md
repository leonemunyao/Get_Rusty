# Livestock Management System

### Introduction

This is a Rust Smart Contract for Livestock Management System which enables a farmerto manage his cattle farm with ease. This system has incorporated the basic CRUD operations. Through the system, a farmer records cattle breed, age =, height, healthrecords, update healthstatus and medical records.

In this system we are assuming a farmer has only bulls and cows to test its functionality. The farmer can be having different breeds like `Brangus`, `Herefold`, `Friasian`, `Zebu` and many more. They use this system to track their different breeds and their specific details.


### Structs

  * `Livestock`: Represents the livesctock with an id, the animal breed, age, height,   health records, health status which can be healthy, sick, critical or recovering, medical records to show list of medications administered to the animal and then an optional parent ID for breeding tracking.

  * `Medication`: This struct represents medication record with name of the medication,and dosage of the medicine.

  * `ParentIds`: This represents the parents IDs for breeding purposes.

  * `HealthAlert`: This struct responsible to get the health status of the animal.

  * `EventLog`: This struct helps in tracking the system changes like creation, updating and deletion.

  * `LivestockManagementSystem`: This struct manages all the livestock records in the system.


### Functions

  #### CRUD Operations

  * `create_animal`: Creates a new animal with a unique ID.

  * `get_animal`: Retrieves details of an animal by ID.

  * `get_all_animals`: Retrieves details of all animals in the system.

  * `update_animal`: Updates the details of an animal by animal ID.

  *  `delete_animal`: Deletes an animal by ID.

  #### Breeding and Pedigree

  * `breed_animals`: Breeds two animals to create a new offspring.

  * `get_pedigree`: Retrieves the family tree of an animal.

  #### Health Management

  * `update_health_status`: updates health status of an animal.

  * `get_health_alerts`: Retrieves all health alerts.

  * `track_medication`: Tracks medication administered to an animal.

  * `get_critical_animals`: Retrieves all animals with a `Critical` health status.

  * `get_sick_animals`: Retrives all animals with a `Sick` health status.

  * `get_recovering_animals`: Retrieves all animals with a `Recovering` health status.

  * `get_healthy_animals`: Retrieves all animals with a `Healthy` health status.

  #### Event Logging

  * `get_event_logs`: Retrieves all the event logs in the systems.

  #### Reporting and Analytics

  * `get_total_animals`: Retrieves the total number of animals in the system.

  * `get_average_age`: Retrieves the average age of all the animals.

  * `get_average_height`: Retrieves the average height of all the animals.

  * `get_animals_per_breed`: Retrieves the number of animals per breed.

  * `get_health_status_statistics`: Retrieves health status statistics of all the animals.



* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```




## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
