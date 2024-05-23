pub trait Esprimo {
    fn es_primo(&self) -> bool;
}

impl Esprimo for i32 {
    fn es_primo(&self) -> bool {
        if *self == 0 || *self == 1 {
            return false;
        }
        for i in 2..*self {
            if *self % i == 0 {
                return false;
            }
        }
        true
    }
}

fn cant_primos<T: Esprimo>(vec: Vec<T>) -> i32 {
    vec.iter().filter(|&x| x.es_primo()).count() as i32
}

#[test]
fn test_es_primo() {
    assert_eq!(1.es_primo(), false);
    assert_eq!(2.es_primo(), true);
    assert_eq!(3.es_primo(), true);
    assert_eq!(4.es_primo(), false);
    assert_eq!(5.es_primo(), true);
    assert_eq!(6.es_primo(), false);
    assert_eq!(7.es_primo(), true);
    assert_eq!(8.es_primo(), false);
    assert_eq!(9.es_primo(), false);
    assert_eq!(10.es_primo(), false);
}

#[test]
fn test_cant_primos() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(cant_primos(vec), 4);
}

#[test]
fn test_cant_primos_sin_primos() {
    let vec = vec![4, 6, 8, 10];
    assert_eq!(cant_primos(vec), 0);
}
