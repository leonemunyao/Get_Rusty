#[macro_use]
extern crate serde;
use candid::{Decode, Encode};   // Serialization format to define the internet canisters
use candid::CandidType;
use ic_cdk::api::time;       //Provide method to allow rust programs to interact with the Internet Computer blockchain API.
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};    //library that offers a set of data structures that remain stable across upgrades.
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};


// Defining Memory and ID Cell

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;  // IdCell responds for holding the current ID of the message 


// Defining Message Struct

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Message {
    id: u64,
    title: String,
    body: String,
    attachment_url: String,
    created_at: u64,
    updated_at: Option<u64>,
}

//Implementing Storable and BoundedStorable

impl Storable for Message {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}


// Setting up Thread Local Variables
// These are variables that are local to the current thread. We use RefCell to manage canister states allowing us to access it from anywhere in our code.


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE: RefCell<StableBTreeMap<u64, Message, Memory>> =
        RefCell:: new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        ));
}


// Setting Up Message Payload
// Message payload is used when adding or updating messagges includes fields for the title, body and attachment URL.

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct MessagePayload {
    title: String,
    body: String,
    attachment_url: String,
}


// We Move on to handling the CRUD operations to the message.

// 1) get_message -- retrieves a message from our canister storage.

#[ic_cdk::query]
fn get_message(id: u64) -> Result<Message, Error> {
    match _get_message(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a message with id={} not found", id),
        }),
    }
}

// _get_message is a helper. It accepsts an id as a reference and returns an `Option<Message>`
// Retrieves the message from the canister storage using the STORAGE thread local varibale.

fn _get_message(id: &u64) -> Option<Message> {
    STORAGE.with(|s| s.borrow().get(id))
}


//  add_message Function. Adding a new message to the canister storage.

#[ic_cdk::update]
fn add_message(message: MessagePayload) -> Option<Message> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");                             // add_message function takes a message of the type MessagePayload as input and returns an Option<Message>
    let message = Message {
        id,
        title: message.title,
        body: message.body,
        attachment_url: message.attachment_url,
        created_at: time(),
        updated_at: None,
    };
    do_insert(&message);
    Some(message)
}


// do_insert message is a helper function used inside the add_message function.
// helper method to perfom insert.

fn do_insert(message: &Message) {
    STORAGE.with(|service| service.borrow_mut().insert(message.id, message.clone()));
}

//  update_message function

#[ic_cdk::update]
fn update_message(id: u64, payload: MessagePayload) -> Result<Message, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut message) => {
            message.attachment_url = payload.attachment_url;
            message.body = payload.body;
            message.title = payload.title;
            message.updated_at = Some(time());
            do_insert(&message);
            Ok(message)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a message with id={}. message not found",
                id
            ),
        }),
    }
}


// delete_message function. For deleting our canister storage.

#[ic_cdk::update]
fn delete_message(id: u64) -> Result<Message, Error> {
    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!(
                "Couldn't delete a message with id={}. message not found.",
                id
            ),
        }),
    }
}

// enum Error
// This is an Error used to represent errors that may occur when interacting with the canister.

#[derive(CandidType, Deserialize, Serialize)]
enum Error {
    NotFound {msg: String},
}


// Generate the candid interface definitions for our canister.

ic_cdk::export_candid!();

