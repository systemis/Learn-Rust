struct Person {
  first_name: String, 
  last_name: String,
}

impl Person {
  fn new(first: &str, last: &str) -> Person {
    return Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    };
  }

  fn full_name(&self) -> String {
    return format!("{} {}", self.first_name, self.last_name);
  }
}

enum Movement {
  Up, 
  Down, 
  Left, 
  Right, 
}

fn move_person(person: Person, m: Movement) -> String {
  let mut movement_promotion; 
  match m {
    Movement::Up => movement_promotion = "Up".to_string(), 
    Movement::Down => movement_promotion = "Down".to_string(), 
    Movement::Left => movement_promotion = "Left".to_string(), 
    Movement::Right => movement_promotion = "Right".to_string(), 
  }

  return format!("Handle person {} with {} action", person.full_name(), movement_promotion);
}

pub fn run() {
  let person1 = Person::new("Pham", "Stephen");
  println!(
    "First name of person 1 {} and last name of person 1 {}", 
    person1.first_name, 
    person1.last_name
  );

  println!("Full name of person is {}", person1.full_name());
  let action_result = move_person(person1, Movement::Down);
  println!("Action result is {}", action_result)
}