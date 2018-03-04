use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");
    run_hash_maps();
    testing_result();
    propogating_errors().unwrap();

    let number_list = vec![1,2,3,4];
    let biggest_number = generic_largest(&number_list);
    println!("Biggest Number: {:?}", biggest_number);

    let char_list = vec!['a', 'b', 'c'];
    let biggest_char = generic_largest(&char_list);
    println!("Biggest Char: {:?}", biggest_char);

    lifetime_features("boo", "booboo");
}

fn run_hash_maps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    let team_name = String::from("Blue");
    scores.insert(team_name, 10); // Now team_name is no longer valid
    scores.entry(String::from("Green")).or_insert(12); // mut borrow and end
    println!("{:?}", scores);
    let key = String::from("Blue");
    // start imut borrow
    let blue_score = scores.get(&key); // get takes a reference type, otherwise the fn would take ownership of the variable
                                       // Checks if a value exists at key  green, inserts 12 if there esnt one
                                       // HashMap keys could have no value, so they return an Option result, so we need to handle Some
                                       // vs None and unwrap it to its value
    match blue_score {
        Some(_) => println!("Blue Score: {:?}", blue_score.unwrap()),
        None => println!("No score yet!"),
    }

    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
} // end imut borrow

// Can fail if there is no file called "Hello.txt"
fn testing_result() {
    // let f = File::open("Hello.txt").expect("Failed to open Hello.txt"); // shortcut to match &
    // panic
    let f = File::open("Hello.txt");

    // Here we shadow f into the contents of the Result return, our initial goal for f
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => { // Luv match guards, error would be moved so ref
            // Could add another match arm here but expect helper is prettier
            File::create("Hello.txt").expect("Couldn't create Hello.txt")
        },
        Err(_error) => {//same ignore var convention as elixir
            panic!("WAHHHBUFFET")
        },
    };

    println!("{:?}", f);
}

fn propogating_errors() -> Result<String, io::Error> {
    let mut f = File::open("Hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// Generic type needs to be bound by PartialOrd traight b/c of
// And only want to implement for data types that have Copy, stack only values
fn generic_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// We don't knwo whether x or y references get returned, and borrow checker doesn't know how their lifetimes
// relate to eachother, ie wants to protect us against dangling references, thanks bruh
// with the generic lifetime notation 'a the borrow checker can tell that all references have the
// same lifetime
fn lifetime_features<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
