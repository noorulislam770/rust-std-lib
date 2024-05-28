#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn new(name: &str, age: i32) -> Result<Self, String> {
        if age >= 21 {
            return Ok(Self {
                name: name.to_owned(),
                age: age,
            });
        } else {
            return Err("Sorry! Age must be greater than 21".to_owned());
        }
    }
}

fn main() {
    let temp_users: Vec<Result<Adult, String>> =
        vec![Adult::new("Noor", 24), Adult::new("ali", 19)];

    for user in temp_users {
        match user {
            Ok(user) => println!("account created successfully"),
            Err(e) => println!("Error Occured : {:?} ", e),
        }
    }
}
