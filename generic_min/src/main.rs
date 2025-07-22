use std::cmp::Ordering;

fn min<T: Ord>(x1: T, x2: T) -> T {
    match x1.cmp(&x2) {
        Ordering::Less | Ordering::Equal => x1,
        Ordering::Greater => x2,
    }
}

#[test]
fn integers() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
}

#[test]
fn chars() {
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
}

#[test]
fn strings() {
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
