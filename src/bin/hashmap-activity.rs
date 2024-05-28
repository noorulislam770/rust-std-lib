use std::collections::HashMap;
fn main() {
    let mut furnitures_stocks = HashMap::new();

    furnitures_stocks.insert("Chairs", 5);
    furnitures_stocks.insert("Beds", 3);
    furnitures_stocks.insert("Tables", 2);
    furnitures_stocks.insert("Couches", 0);
    let mut total_stock = 0;
    for (item, quantity) in furnitures_stocks.iter() {
        total_stock = total_stock + quantity;
        if quantity <= &0 {
            println!("{} : Out of Stock", item);
        } else {
            println!("Item : {}, quantity : {}", item, quantity);
        }
    }

    println!("The Total stock is {}", total_stock);
}
