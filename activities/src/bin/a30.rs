// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {}
trait Color {}

#[derive(Debug)]
struct Vehicule<B: Body, C: Color> {
  body: B,
  color: C,
}

impl<B: Body, C: Color> Vehicule<B, C> {
  fn new(body: B, color: C) -> Self {
    Self { body, color }
  }
}

#[derive(Debug)]
struct Car;
#[derive(Debug)]
struct Truck;

impl Body for Car {}
impl Body for Truck {}

#[derive(Debug)]
struct Red;
#[derive(Debug)]
struct Blue;

impl Color for Red {}
impl Color for Blue{}


fn main() {
  let blue_truck = Vehicule::new(Truck,Blue);
  let red_car = Vehicule::new(Car, Red);
  println!("{:?}", blue_truck);
  println!("{:?}", red_car);
}
