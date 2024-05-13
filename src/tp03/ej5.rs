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
fn constructor() {
    let producto = Producto::new("Producto1".to_string(), 100.0, 123);
    assert_eq!(producto.nombre, "Producto1".to_string());
    assert_eq!(producto.precioB, 100.0);
    assert_eq!(producto.id, 123);
}

#[test]
fn impuestos() {
    let producto = Producto::new("Producto1".to_string(), 100.0, 123);
    assert_eq!(producto.calcular_impuestos(Some(10.0)), 10.0);
    assert_eq!(producto.calcular_impuestos(None), 0.0);
}

#[test]
fn descuentos() {
    let producto = Producto::new("Producto1".to_string(), 100.0, 123);
    assert_eq!(producto.aplicar_descuento(Some(10.0)), 10.0);
    assert_eq!(producto.aplicar_descuento(None), 0.0);
}

#[test]
fn total() {
    let producto = Producto::new("Producto1".to_string(), 100.0, 123);
    assert_eq!(producto.calcular_precio_total(Some(10.0), Some(20.0)), 90.0);
    assert_eq!(producto.calcular_precio_total(None, Some(20.0)), 80.0);
    assert_eq!(producto.calcular_precio_total(Some(10.0), None), 110.0);
    assert_eq!(producto.calcular_precio_total(None, None), 100.0);
}
