// Topic: Ownership

struct GroceryItem {
    quantity: i32,
    id: i32
}

fn display_quantity(item: GroceryItem) {
    println!("Quantity: {:?}", item.quantity);
}

fn display_id(item: GroceryItem) {
    println!("ID: {:?}", item.id);
}

fn main() {
    let item = GroceryItem {
        quantity: 5,
        id: 1
    };
    display_quantity(&item);
    display_id(item);
}



// wrong usage
// fn display_quantity(item: GroceryItem) {
//     println!("Quantity: {:?}", item.quantity);
// }
//
// fn display_id(item: GroceryItem) {
//     println!("ID: {:?}", item.id);
// }
//
// fn main() {
//     let item = GroceryItem {
//         quantity: 5,
//         id: 1
//     };
//     display_quantity(&item);
//     display_id(item);
// }
