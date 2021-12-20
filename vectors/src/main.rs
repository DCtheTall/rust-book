fn main() {
    // Create a vector, can only store a single type
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Vectors are dynamic lists
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    {
        let v = vec![1, 2, 3];
    } // <- v goes out of scope and is freed along with its elements

    let v = vec![1, 2, 3, 4, 5];

    // Can use subscript operator
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Or .get() method and `match`
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // Causes panic at runtime
    // let does_not_exist = &v[100];
    // Instead use .get(), which returns an Option that is None if the element does not exist
    let does_not_exist = v.get(100);

    // Compiler error, .push() does a mutable borrow of each element, so any immutable borrow beforehand is now invalid
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is {}", first);

    // Iterating over vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Modifying a vector in a `for` loop
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // To store multiple types in a vector, use an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
