fn main() {
    println!("Hello, world!");
}

#[test]
fn simple_test() {
    assert_eq!(2 * 2, 4);
}

#[test]
fn test_two() {
    assert_eq!(2 * 123, 246);
}
