use std::collections::hash_map::HashMap;

// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

fn main() {
  let mut store = HashMap::new();
  store.insert("Chairs", 5);
  store.insert("Beds", 3);
  store.insert("Tables", 2);
  store.insert("Couches", 0);

  let mut total = 0;

  for (k, v) in store.iter() {
    total += v;
    let stock_count = if v == &0 {
      "out of stock".to_owned()
    } else {
      format!("{:?}", v)
    };
    println!("item: {:?}, stock: {:?}", k, stock_count);
  }
  println!("total: {:?}", total);
}
