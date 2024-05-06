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
fn testt() {
    let rect = Rectangulo::new(10.5, 10.5);
    print!("{}", rect.es_cuadrado());
}
