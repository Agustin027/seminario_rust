pub fn es_primo(numero: u32) -> bool {
    for i in 2..=numero / 2 {
        if numero % i == 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_es_primo() {
    assert_eq!(true, es_primo(7));
}
