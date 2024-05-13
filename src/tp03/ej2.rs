struct Rectangulo {
    longitud: f32,
    ancho: f32,
}

impl Rectangulo {
    fn new(longitud: f32, ancho: f32) -> Self {
        Rectangulo { longitud, ancho }
    }

    fn calcular_area(&self) -> f32 {
        self.longitud * self.ancho
    }

    fn calcular_perimetro(&self) -> f32 {
        self.longitud * 2.0 + self.ancho * 2.0
    }

    fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

#[test]
fn constructor() {
    let rectangulo = Rectangulo::new(10.0, 5.0);
    assert_eq!(rectangulo.longitud, 10.0);
    assert_eq!(rectangulo.ancho, 5.0);
}

#[test]
fn area() {
    let rectangulo = Rectangulo::new(10.0, 5.0);
    assert_eq!(rectangulo.calcular_area(), 50.0);
}

#[test]
fn perimetro() {
    let rectangulo = Rectangulo::new(10.0, 5.0);
    assert_eq!(rectangulo.calcular_perimetro(), 30.0);
}

#[test]
fn cuadrado() {
    let rectangulo = Rectangulo::new(10.0, 5.0);
    assert_eq!(rectangulo.es_cuadrado(), false);
    let cuadrado = Rectangulo::new(10.0, 10.0);
    assert_eq!(cuadrado.es_cuadrado(), true);
}
#[test] 
fn area_cero() {
    let rectangulo = Rectangulo::new(0.0, 5.0);
    assert_eq!(rectangulo.calcular_area(), 0.0);
}
