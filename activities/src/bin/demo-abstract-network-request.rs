#![allow(dead_code)]

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Person {
  name: String,
  country: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PersonResponse {
  status: String,
  code: u16,
  total: u64,
  data: Vec<Person>,
}

trait DataFetcher {
  fn get_person(&self) -> Result<PersonResponse>;
}

#[derive(Default)]
struct ReqwestFetcher {
  client: reqwest::blocking::Client,
}

impl DataFetcher for ReqwestFetcher {
  fn get_person(&self) -> Result<PersonResponse> {
    let response: PersonResponse = self
      .client
      .get("https://fakerapi.it/api/v1/custom?_quantity=1&name=name&country=country")
      .send()?
      .json()?;

    Ok(response)
  }
}

// fn get_person(client: &reqwest::blocking::Client) -> Result<PersonResponse> {
//     let response: PersonResponse = client
//         .get("https://fakerapi.it/api/v1/custom?_quantity=1&name=name&country=country")
//         .send()?
//         .json()?;

//     Ok(response)
// }

// #[derive(Debug, Default)]
struct App {
  client: Box<dyn DataFetcher>,
}

impl Default for App {
  fn default() -> Self {
    Self {
      client: Box::new(ReqwestFetcher::default()),
    }
  }
}

impl App {
  pub fn fetch_person(&self) -> Result<PersonResponse> {
    self.client.get_person()
  }
}

fn main() -> Result<()> {
  let app = App::default();
  let person = app.fetch_person();
  dbg!(&person);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  struct FakeFetcher;

  impl DataFetcher for FakeFetcher {
    fn get_person(&self) -> Result<PersonResponse> {
      Ok(PersonResponse {
        code: 200,
        status: "OK".to_string(),
        total: 1,
        data: vec![Person {
          name: "Ricardo David".to_string(),
          country: "test country".to_string(),
        }],
      })
    }
  }

  #[test]
  fn fetched_data() {
    let app = App {
        client: Box::new(FakeFetcher)
    };
    let person = app.fetch_person();
    //dbg!(&person);
    assert!(person.is_ok())
  }
}
