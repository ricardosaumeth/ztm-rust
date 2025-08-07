// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Material {
  fn cost(&self) -> f64;
}

struct Carpet {
  price_per_m2: f64,
  square_meters: f64,
}

impl Material for Carpet {
  fn cost(&self) -> f64 {
    self.price_per_m2 * self.square_meters
  }
}

struct Tile {
  price_per_m2: f64,
  square_meters: f64,
}

impl Material for Tile {
  fn cost(&self) -> f64 {
    self.price_per_m2 * self.square_meters
  }
}

struct Wood {
  price_per_m2: f64,
  square_meters: f64,
}

impl Material for Wood {
  fn cost(&self) -> f64 {
    self.price_per_m2 * self.square_meters
  }
}

fn calculate_cost(materials: &Vec<Box<dyn Material>>) -> f64 {
  materials.iter().map(|material| material.cost()).sum()
}

fn main() {
  let carpet_price_m2 = Box::new(Carpet {
    price_per_m2: 10.0,
    square_meters: 10.0,
  });
  let title_price_m2 = Box::new(Tile {
    price_per_m2: 15.0,
    square_meters: 10.0,
  });
  let wood_price_m2 = Box::new(Wood {
    price_per_m2: 20.0,
    square_meters: 10.0,
  });

  let materials: Vec<Box<dyn Material>> = vec![carpet_price_m2, title_price_m2, wood_price_m2];
  println!("{:?}", calculate_cost(&materials))
}
