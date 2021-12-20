fn main() {
    let mut s = String::from("hello world");
    let i = first_word_index(&s);
    println!("i is {}", i);
    // i depended on s, but when s clears its data, i remains unchanged
    s.clear();
    println!("i is {}", i);

    s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    // Could also equivalently write
    let _hello = &s[..5];
    let _world = &s[6..];

    let hello = first_word(&s);
    println!("first word is {}", hello);

    // Compiler error since the println! call makes an immutable borrow of `s`
    // after `s.clear()` already created a mutable reference to `s`.
    // Rust disallows this
    // s.clear();
    // println!("first word is: {}", hello);

    let hello1 = better_first_word(&s);
    let hello2 = better_first_word("hello world");
    assert_eq!(hello1, hello2);

    // Arrays can also have slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// This function returns a string slice, denoted &str
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    // Eqivalent to &s[0..s.len()]
    &s[..]
}

// We can rewrite `first_word` to use a string slice as a parameter
// This way we can use a string literal or String argument when calling the function
fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    // Eqivalent to &s[0..s.len()]
    &s[..]
}
