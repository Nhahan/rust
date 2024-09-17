// Topic: HashMap

use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("apples", 10);
    stock.insert("bananas", 5);
    stock.insert("pears", 7);
    stock.insert("plums", 3);

    let mut total_stock = 0;

    for (item, quantity) in stock.iter() {
        let stock_count = if quantity == &0 {
            "out of stock".to_string()
        } else {
            quantity.to_string()
        };
        total_stock += quantity;
        println!("{}: {}", item, stock_count);
    }

    println!("total stock: {}", total_stock);
}
