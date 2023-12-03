#[test]
fn destructuring_test() {
    let mut a = 3;
    let b;
    assert_eq!(a, 3);
    (a, b) = (1, 2);
    //(a, let b) = (1, 2); did not work
    assert_eq!(a, 1);
    assert_eq!(b, 2);

}
