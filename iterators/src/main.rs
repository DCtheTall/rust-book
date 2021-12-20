fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // No need to mark iterator `mut` for `for` loop since it will take ownership
    for val in v1_iter {
        println!("{}", val);
    }

    // Note to call `next` the iterator must be marked `mut`
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // `sum` takes ownership of `v1_iter` so it does not need to be `mut`
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // `map` method creates a new, parallel iterator
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // Compiler error without type annotation
    let v2: Vec<i32> = v1_iter.map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
