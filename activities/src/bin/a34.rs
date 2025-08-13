// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Debug, Clone)]

#[derive(Copy, Clone)]
struct LuggageId(usize);
struct Lugagge(LuggageId);
struct CheckIn(LuggageId);
struct OnLoading(LuggageId);
struct Offloading(LuggageId);
struct AwaitingPickup(LuggageId);
#[derive(Debug, Clone)]
struct EndCustody(LuggageId);

impl Lugagge {
  fn new(id: LuggageId) -> Self {
    Lugagge(id)
  }

  fn check_in(self) -> CheckIn {
    CheckIn(self.0)
  }
}

impl CheckIn {
  fn onloading(self) -> OnLoading {
    OnLoading(self.0)
  }
}

impl OnLoading {
  fn offloading(self) -> Offloading {
    Offloading(self.0)
  }
}

impl Offloading {
  fn carousel(self) -> AwaitingPickup {
    AwaitingPickup(self.0)
  }
}

impl AwaitingPickup {
  fn pickup(self) -> (Lugagge, EndCustody) {
    (Lugagge(self.0), EndCustody(self.0))
  }
}

fn main() {
  let id = LuggageId(1);
  let luggage = Lugagge::new(id);
  let luggage = luggage.check_in().onloading().offloading().carousel();
  let (luggage, _) = luggage.pickup();

}
