pub fn incrementar(num: &mut f64) {
    *num += 1.0;
}

#[test]
fn test_incrementar() {
    let mut num = 1.1;
    incrementar(&mut num);
    assert_eq!(num, 2.1);
}
