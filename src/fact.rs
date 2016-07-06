pub fn fact(n: i32) -> i32 {
    if n == 0 { 1 }
    else { n * fact(n-1) }
}

#[test]
fn test_fact() {
    assert_eq!(720, fact(6));
}
