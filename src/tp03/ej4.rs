struct Triangulo {
    lado1: f64,
    lado2: f64,
    lado3: f64,
}
impl Triangulo {
    fn new(lado1: f64, lado2: f64, lado3: f64) -> Self {
        Triangulo {
            lado1,
            lado2,
            lado3,
        }
    }
    fn determinar_tipo(&self) -> String {
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            "equilátero".to_string()
        } else if self.lado1 == self.lado2 || self.lado1 == self.lado3 || self.lado2 == self.lado3 {
            "isósceles".to_string()
        } else {
            "escaleno".to_string()
        }
    }

    fn calcular_perimetro(&self) -> f64 {
        self.lado1 + self.lado2 + self.lado3
    }

    fn calcular_area(&self) -> f64 {
        let s = self.calcular_perimetro() / 2.0;
        (s * (s - self.lado1) * (s - self.lado2) * (s - self.lado3)).sqrt()
    }
}
