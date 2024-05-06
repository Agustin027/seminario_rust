pub fn es_par(num: i32) -> bool {
    num % 2 == 0
}

#[test]
fn test_es_par() {
    assert_eq!(true, es_par(3));
}
