struct Producto {
    nombre: String,
    precioB: f64,
    id: u32,
}

impl Producto {
    fn new(nombre: String, precioB: f64, id: u32) -> Self {
        Producto {
            nombre,
            precioB,
            id,
        }
    }

    fn calcular_impuestos(&self, impuesto: Option<f64>) -> f64 {
        match impuesto {
            Some(porcentaje) => self.precioB * porcentaje / 100.0,
            None => 0.0,
        }
    }

    fn aplicar_descuento(&self, descuento: Option<f64>) -> f64 {
        match descuento {
            Some(porcentaje) => self.precioB * porcentaje / 100.0,
            None => 0.0,
        }
    }

    fn calcular_precio_total(&self, impuesto: Option<f64>, descuento: Option<f64>) -> f64 {
        let impuestos = self.calcular_impuestos(impuesto);
        let descuentos = self.aplicar_descuento(descuento);
        self.precioB + impuestos - descuentos
    }
}

#[test]
fn test() {
    let producto = Producto::new(String::from("Producto1"), 100.0, 12345);

    println!(
        "Precio total: ${}",
        producto.calcular_precio_total(Some(10.0), Some(20.0))
    );
}
