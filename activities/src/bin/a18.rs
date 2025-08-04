// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
  age: u8,
  name: String,
}
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
impl Adult {
  fn new(name: &str, age: u8) -> Result<Self, String> {
    if age >= 21 {
      Ok(Self {
        age,
        name: name.to_string(),
      })
    } else {
      Err(format!("Error: age {} must be over 21", age))
    }
  }
}

// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

fn main() {
  let a = Adult::new("Ricardo Manuel", 45);
  let b = Adult::new("Ricardo David", 4);
  // print!("{:#?}", a);
  // print!("{:#?}", b)
  match a {
    Ok(b) => print!("{:#?}", b),
    Err(e) => print!("{:#?}", e),
  }

  match b {
    Ok(b) => print!("{:#?}", b),
    Err(e) => print!("{:#?}", e),
  }
}
