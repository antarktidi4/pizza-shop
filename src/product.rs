#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ProductType {
    Pizza,
    Drink,
}

impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProductType::Pizza => write!(f, "Pizza"),
            ProductType::Drink => write!(f, "Drink"),
        }
    }
}

pub trait ProductTrait {
    fn new(product_type: ProductType, name: String, price: i32, discount: i32) -> Self;
    fn get_type(&self) -> ProductType;
    fn get_name(&self) -> String;
    fn get_price(&self) -> i32;
    fn get_discount(&self) -> i32;
    fn get_final_price(&self) -> i32;
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Product {
    product_type: ProductType,
    name: String,
    price: i32,
    discount: i32,
}

impl ProductTrait for Product {
    fn new(product_type: ProductType, name: String, price: i32, discount: i32) -> Self {
        Product {
            product_type,
            name,
            price,
            discount,
        }
    }

    fn get_type(&self) -> ProductType {
        self.product_type.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_price(&self) -> i32 {
        self.price.clone()
    }
    fn get_discount(&self) -> i32 {
        self.discount.clone()
    }

    fn get_final_price(&self) -> i32 {
        let price = &self.price;
        let discount = &self.discount;
        ((*price as f32) - (((*price as f32) / 100.0) * (*discount as f32))) as i32
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "product: {} type: {} price: {} discount: {}%",
            self.name, self.product_type, self.price, self.discount
        )
    }
}
