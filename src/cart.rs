use crate::product::*;
use std::collections::HashMap;

pub trait CartTrait {
    fn new() -> Self;
    fn add_to_cart(&mut self, product: Product, count: i32);
    fn get_items(&self) -> HashMap<Product, i32>;
}

pub struct Cart {
    items: HashMap<Product, i32>,
}

impl CartTrait for Cart {
    fn new() -> Self {
        Cart {
            items: HashMap::new(),
        }
    }

    fn add_to_cart(&mut self, product: Product, count: i32) {
        self.items.insert(product, count);
    }

    fn get_items(&self) -> HashMap<Product, i32> {
        self.items.clone()
    }
}
