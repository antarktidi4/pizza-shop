mod cart;
mod printer;
mod product;
mod shop;

use crate::cart::*;
use crate::shop::*;

fn main() {
    let shop = Shop::init();
    let mut cart = Cart::new();
    printer::print_menu(&shop);
    shop.add_items_to_cart(&mut cart);
    printer::print_check(&cart);
}
