struct Dimentions {
  width: f64,
  height: f64,
  depth: f64,
}

trait Convey {
  fn weight(&self) -> f64;
  fn dimentions(&self) -> Dimentions;
}

struct ConveyorBelt<T: Convey> {
  pub item: Vec<T>,
}

impl<T: Convey> ConveyorBelt<T> {
  pub fn add(&mut self, item: T) {
    self.item.push(item);
  }
}

struct CarPark {
  width: f64,
  height: f64,
  depth: f64,
  weight: f64,
  part_number: String,
}

impl Default for CarPark {
  fn default() -> Self {
    Self {
      width: 5.0,
      height: 1.0,
      depth: 2.0,
      weight: 3.0,
      part_number: "abc".to_owned(),
    }
  }
}

impl Convey for CarPark {
  fn weight(&self) -> f64 {
    self.weight
  }

  fn dimentions(&self) -> Dimentions {
    Dimentions {
      width: self.width,
      depth: self.depth,
      height: self.height,
    }
  }
}

fn main() {
  let mut belt = ConveyorBelt {item: vec![]};
  belt.add(CarPark::default());
}
