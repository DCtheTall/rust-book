use std::slice;

fn main() {
    // Raw pointers
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Setting the address of a pointer directly
    let address = 0x012345usize;
    let r = address as *const i32;

    // You must use an `unsafe` block to deref a raw pointer
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *const i32;
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // You must use an `unsafe` block to call an unsafe function or method
    unsafe {
        dangerous();
    }

    // Example of the `split_at_mut` method
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // You can use `unsafe` to call functions in the C standard library
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("absolute value of -3 is {}", abs(-3));
    }

    // Immutable statics can be used in safe Rust
    println!("HELLO_WORLD is {}", HELLO_WORLD);

    // Mutable statics require `unsafe` for reads and writes
    unsafe {
        COUNTER += 3;
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

// Cannot implement split_at_mut using safe Rust
// Compiler error, multiple mutable borrows
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut[i32]) {
//     let len = slice.len();
//     assert!(mid <= len);
//     (&mut slice[..mid], &mut slice[mid..])
// }

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

// Traits are unsafe when at least one of its methods has an invariant the
// compiler can't verify
unsafe trait Foo {
    // methods...
}
unsafe impl Foo for i32 {
    // implementations here...
}
