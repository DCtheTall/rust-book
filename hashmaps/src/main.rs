use std::collections::HashMap;

fn main() {
    // Insert by individual key/value
    let mut scores = HashMap::new();
    // insert() takes ownership of the key and value
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // Insert parallel vectors of keys and values
    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Accessing elements in a map
    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    match score {
        Some(x) => println!("{}", x),
        None => println!("not found"),
    }

    // Iterate over keys and values
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // Inserts "Yellow" -> 50 since "Yellow" is not in the map
    scores.entry(String::from("Yellow")).or_insert(50);
    // Does not insert "Blue" -> 50 since "Blue" is already in the map
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating existing values in the map
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
