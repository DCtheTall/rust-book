fn main() {
    // type String
    let s = String::new();

    // type &str
    let data = "initial_contents";
    // Convert string literal to String
    let s: String = data.to_string();
    println!("{}", s);
    // Initialize a string
    let s = String::from("initial contents");

    // String supports UTF8 characters
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    // Compiler error, push_str does not take a String so it does not take ownership of it
    // s1.push_str(s2);
    // push_str() can borrow the String so the String can be used later
    s1.push_str(&s2);
    println!("{}", s2);

    let mut s = String::from("lo");
    // push() lets you append a character
    s.push('l');
    println!("{}", s);

    // Use + operator to concatenate strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Concatenation takes ownership of the left operand and borrows the right operand
    let s3 = s1 + &s2;
    // Compiler error, s3 now owns the String stored in s1
    // println!("{}", s1);
    println!("{}", s3);

    // For concatenation multiple strings...
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    // Compiler error
    // println!("{}", s1);

    // ...you can use the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    // format! does not take ownership of any parameters
    println!("{}", s1);

    // You cannot access a string character using integer indexing since it uses UTF8
    let s = String::from("Здравствуйте");
    // let c = &s[0];
    // Index slicing is ok since you can specify a range of bytes
    let slice = &s[0..4];

    // Iterating over string characters
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // over string bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
