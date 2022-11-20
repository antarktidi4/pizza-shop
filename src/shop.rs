use crate::cart::*;
use crate::product::*;
use std::io::Write;

pub trait ShopTrait {
    fn init() -> Self;
    fn get_menu(&self) -> Vec<Product>;
    fn add_item_to_cart(&self, product: Product, cart: &mut Cart);
    fn add_items_to_cart(&self, cart: &mut Cart);
}

#[derive(Clone)]
pub struct Shop {
    goods: Vec<Product>,
}

impl ShopTrait for Shop {
    fn init() -> Self {
        let goods = vec![
            Product::new(ProductType::Pizza, String::from("Italian Pizza"), 100, 10),
            Product::new(ProductType::Pizza, String::from("American Pizza"), 120, 0),
            Product::new(
                ProductType::Pizza,
                String::from("Expensive Pizza with 100% disc."),
                1000,
                100,
            ),
            Product::new(ProductType::Pizza, String::from("Classic Pizza"), 80, 0),
            Product::new(ProductType::Pizza, String::from("Pizza by Shrek"), 98, 5),
            Product::new(ProductType::Drink, String::from("Coca Cola"), 64, 0),
            Product::new(ProductType::Drink, String::from("Pepsi Cola"), 67, 3),
        ];

        Shop { goods }
    }

    fn get_menu(&self) -> Vec<Product> {
        self.goods.clone()
    }

    fn add_item_to_cart(&self, product: Product, cart: &mut Cart) {
        let mut count: String = String::new();
        print!("amount of {}:", product.get_name());
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut count)
            .expect("Something went wrong...");
        let i32_count = match count.trim_end().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("NaN error, skipping this item...");
                return;
            }
        };
        if i32_count > 0 {
            cart.add_to_cart(product, i32_count);
        }
    }

    fn add_items_to_cart(&self, cart: &mut Cart) {
        for product in self.goods.iter() {
            self.add_item_to_cart(product.clone(), cart);
        }
    }
}
