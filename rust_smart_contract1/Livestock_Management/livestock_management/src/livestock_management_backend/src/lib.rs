#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMmemoryImpl, StableTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMmemoryImpl>;
type IdCell = Cell<u64, Memory>;


// Define the livestock struct 
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Livestock {
    id: u64;
    breed: String;
    age: u8;
    height: f32; 
    healthrecords: String;
    created_at: u64;
    updated_at: Option<u64>;
}