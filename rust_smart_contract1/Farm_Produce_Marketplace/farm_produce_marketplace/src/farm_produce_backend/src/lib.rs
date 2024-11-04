// Farm Produce Marketplace Backend.

use ic_cdk::query;
// import the necessary modules
use serde::{Deserialize, Serialize};
use candid::{CandidType, Principal};
use ic_cdk_macros::{init, update};



// Define the contract struct
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Marketplace {
    products: Vec<Product>,
}

// Define the Product struct
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Product {
    id: u64,
    name: String,
    description: String,
    quantity: u64,
    price: u128,
    seller: Principal,
}

#[init]
pub fn init() {
    ic_cdk::storage::stable_save((Marketplace { products: Vec::new() },)).unwrap();
}

#[update]
pub fn create_product(name: String, description: String, quantity: u64, price: u128) {
    let (mut marketplace,): (Marketplace,) = ic_cdk::storage::stable_restore().unwrap();
    let product = Product {
        id: marketplace.products.len() as u64,
        name,
        description,
        quantity,
        price,
        seller: ic_cdk::caller(),
    };
    marketplace.products.push(product);
    ic_cdk::storage::stable_save((marketplace,)).unwrap();
}


// Read function to get all products. Buyers will have the ability to view available products including their current prices and quantities.
// Read function gets the products by name.

#[query]
pub fn get_products(name: String) -> Vec<Product> {
    let (marketplace,): (Marketplace,) = ic_cdk::storage::stable_restore().unwrap();
    let name = name.trim();
    let filtered_products: Vec<Product> = marketplace.products
        .into_iter()
        .filter(|product| product.name.trim() == name)
        .collect();
    ic_cdk::println!("Filtered products: {:?}", filtered_products);
    filtered_products
}


// Export the contract
ic_cdk::export_candid!();