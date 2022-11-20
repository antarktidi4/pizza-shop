use crate::cart::*;
use crate::product::*;
use crate::shop::*;

pub fn print_check(cart: &Cart) {
    println!("---- check ----");
    for (product, count) in cart.get_items().iter() {
        println!(
            "{} - {}$ ({} pcs.)",
            product.get_name(),
            product.get_final_price() * count,
            count
        );
    }
    println!("---------------");
    let price_with_discount = calc_price(cart, true);
    let price_without_discount = calc_price(cart, false);
    let economy = price_without_discount - price_with_discount;
    println!("to pay: {} (-{}$)", price_with_discount, economy);
    println!("---------------");
}

fn calc_price(cart: &Cart, discount: bool) -> i32 {
    let mut price_with_discount: i32 = 0;
    for (product, count) in cart.get_items().iter() {
        let price = if discount {
            product.get_final_price()
        } else {
            product.get_price()
        };
        price_with_discount += price * count;
    }
    price_with_discount
}

pub fn print_menu(shop: &Shop) {
    println!("=== menu ===");
    for product in shop.get_menu().iter() {
        println!("{}", product);
    }
    println!("============");
}
