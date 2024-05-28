use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct Contents {
    content: String,
}
fn main() {
    // let mut people = HashMap::new();
    // people.insert("Susam", 21);
    // people.insert("Ali", 13);
    // people.insert("John", 14);
    // people.insert("Mohsin", 22);
    // people.insert("Hamza", 23);

    // match people.get("John") {
    //     Some(age) => println!("age = {:?}", age),
    //     None => println!("not found "),
    // }

    // for (person, age) in people.iter() {
    //     println!("Person = {:?}, age = {:?}", person, age);
    // }
    // for person in people.keys() {
    //     println!("Person = {:?}", person);
    // }
    // for age in people.iter() {
    //     println!("age = {:?}", age);
    // }

    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "stuff".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "books".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "shorts".to_owned(),
        },
    );

    for (locker_number, contents) in lockers {
        println!("number : {:?}, content : {:?}", locker_number, contents)
    }
}
