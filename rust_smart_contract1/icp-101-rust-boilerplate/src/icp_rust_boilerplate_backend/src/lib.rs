#[macro_use]
extern create serde;
use candid::{Decode,  Encode};   // Serialization format to define the internet canisters
use ic_cdk::api::time;       //Provide method to allow rust programs to interact 
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableTreeMap, Storable};
use std::{borrow::Cow, cell::Refcell};



