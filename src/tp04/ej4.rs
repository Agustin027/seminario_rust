use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Categoria {
    Alimentos,
    Bebidas,
    Limpieza,
    Ropa,
    Tecnologia,
}
#[derive(Debug, PartialEq, Clone)]
struct Producto {
    nombre: String,
    categoria: Categoria,
    precio: f32,
}

impl Producto {
    fn new(nombre: String, categoria: Categoria, precio: f32) -> Self {
        Producto {
            nombre,
            categoria,
            precio,
        }
    }

    fn aplicar_desc(&self) -> f32 {
        match self.categoria {
            Categoria::Alimentos => self.precio * 0.9,
            Categoria::Bebidas => self.precio * 0.8,
            Categoria::Limpieza => self.precio * 0.85,
            Categoria::Ropa => self.precio * 0.95,
            Categoria::Tecnologia => self.precio * 0.75,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Cliente {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
    newsletter: Option<String>,
}
#[derive(Debug, PartialEq, Clone)]
struct Vendedor {
    legajo: String,
    antiguedad: i32,
    salario: f32,
}
impl Vendedor {
    fn new(legajo: String, antiguedad: i32, salario: f32) -> Self {
        Vendedor {
            legajo,
            antiguedad,
            salario,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MedioPago {
    Efectivo,
    Tarjeta,
    Debito,
    Transferencia,
}

struct SisteamaVentas {
    ventas: Vec<Venta>,
}

impl SisteamaVentas {
    fn new(vec: Vec<Venta>) -> Self {
        SisteamaVentas { ventas: vec }
    }

    fn crear_venta(
        &mut self,
        fecha: String,
        productos: Vec<Producto>,
        cliente: Cliente,
        vendedor: Vendedor,
        medio_pago: MedioPago,
    ) {
        let venta = Venta::crear_venta(fecha, productos, cliente, vendedor, medio_pago);
        self.ventas.push(venta);
    }

    fn control_ventas_categ(&self) -> Vec<i32> {
        let mut vec = vec![0, 0, 0, 0, 0];
        for venta in &self.ventas {
            for producto in &venta.productos {
                match producto.categoria {
                    Categoria::Alimentos => vec[0] += 1,
                    Categoria::Bebidas => vec[1] += 1,
                    Categoria::Limpieza => vec[2] += 1,
                    Categoria::Ropa => vec[3] += 1,
                    Categoria::Tecnologia => vec[4] += 1,
                }
            }
        }
        return vec;
    }
    fn control_ventas_vendedor(&self) -> HashMap<String, i32> {
        let mut contador = HashMap::new();

        for venta in &self.ventas {
            let entry = contador.entry(venta.vendedor.legajo.clone()).or_insert(0);
            *entry += 1;
        }
        return contador;
    }
}
#[derive(Debug, PartialEq, Clone)]
struct Venta {
    fecha: String,
    productos: Vec<Producto>,
    cliente: Cliente,
    vendedor: Vendedor,
    medio_pago: MedioPago,
}

impl Venta {
    fn crear_venta(
        fecha: String,
        productos: Vec<Producto>,
        cliente: Cliente,
        vendedor: Vendedor,
        medio_pago: MedioPago,
    ) -> Self {
        Venta {
            fecha,
            productos,
            cliente,
            vendedor,
            medio_pago,
        }
    }
    fn calcular_precio_final(&self) -> f32 {
        let mut total = 0.0;

        for producto in &self.productos {
            total += producto.aplicar_desc();
        }

        if self.cliente.newsletter.is_some() {
            total *= 0.85;
        }

        return total;
    }
}

#[test]
fn test_constructor_producto() {
    let producto = Producto::new("Coca-Cola".to_string(), Categoria::Bebidas, 100.0);
    assert_eq!(producto.nombre, "Coca-Cola".to_string());
    assert_eq!(producto.categoria, Categoria::Bebidas);
    assert_eq!(producto.precio, 100.0);
}

#[test]
fn test_descuento() {
    let producto = Producto::new("Coca-Cola".to_string(), Categoria::Bebidas, 100.0);
    assert_eq!(producto.aplicar_desc(), 80.0);
}

#[test]
fn test_constructor_cliente() {
    let cliente = Cliente {
        nombre: "Juan".to_string(),
        apellido: "Perez".to_string(),
        direccion: "Av. Siempre Viva 123".to_string(),
        dni: "12345678".to_string(),
        newsletter: Some("mail de prueba".to_string()),
    };
    assert_eq!(cliente.nombre, "Juan".to_string());
    assert_eq!(cliente.apellido, "Perez".to_string());
    assert_eq!(cliente.direccion, "Av. Siempre Viva 123".to_string());
    assert_eq!(cliente.dni, "12345678".to_string());
    assert_eq!(cliente.newsletter, Some("mail de prueba".to_string()));
}
#[test]
fn test_constructor_vendedor() {
    let vendedor = Vendedor::new("1234".to_string(), 5, 10000.0);
    assert_eq!(vendedor.legajo, "1234".to_string());
    assert_eq!(vendedor.antiguedad, 5);
    assert_eq!(vendedor.salario, 10000.0);
}

#[test]
fn test_constructor_venta() {
    let cliente = Cliente {
        nombre: "Juan".to_string(),
        apellido: "Perez".to_string(),
        direccion: "Av. Siempre Viva 123".to_string(),
        dni: "12345678".to_string(),
        newsletter: Some("mail de prueba".to_string()),
    };
    let vendedor = Vendedor::new("1234".to_string(), 5, 10000.0);
    let producto = Producto::new("Coca-Cola".to_string(), Categoria::Bebidas, 100.0);
    let venta = Venta::crear_venta(
        "12/12/2020".to_string(),
        vec![producto.clone()],
        cliente.clone(),
        vendedor.clone(),
        MedioPago::Tarjeta,
    );

    assert_eq!(venta.fecha, "12/12/2020".to_string());
    assert_eq!(venta.productos, vec![producto.clone()]);
    assert_eq!(venta.cliente, cliente.clone());
    assert_eq!(venta.vendedor, vendedor.clone());
    assert_eq!(venta.medio_pago, MedioPago::Tarjeta);
}

#[test]
fn test_precio_final() {
    let cliente = Cliente {
        nombre: "Juan".to_string(),
        apellido: "Perez".to_string(),
        direccion: "Av. Siempre Viva 123".to_string(),
        dni: "12345678".to_string(),
        newsletter: Some("mail de prueba".to_string()),
    };
    let vendedor = Vendedor::new("1234".to_string(), 5, 10000.0);
    let producto = Producto::new("Coca-Cola".to_string(), Categoria::Bebidas, 100.0);
    let venta = Venta::crear_venta(
        "12/12/2020".to_string(),
        vec![producto],
        cliente,
        vendedor,
        MedioPago::Tarjeta,
    );
    assert_eq!(venta.calcular_precio_final(), 68.0);
}

#[test]
fn test_control_ventas_vendedor() {
    // Crear algunos productos
    let producto1 = Producto::new("Producto 1".to_string(), Categoria::Alimentos, 100.0);
    let producto2 = Producto::new("Producto 2".to_string(), Categoria::Bebidas, 50.0);
    let producto3 = Producto::new("Producto 3".to_string(), Categoria::Tecnologia, 200.0);

    // Crear algunos vendedores
    let vendedor1 = Vendedor::new("1234".to_string(), 5, 10000.0);
    let vendedor2 = Vendedor::new("5678".to_string(), 3, 15000.0);

    // Crear un cliente
    let cliente = Cliente {
        nombre: "Juan".to_string(),
        apellido: "Perez".to_string(),
        direccion: "Av. Siempre Viva 123".to_string(),
        dni: "12345678".to_string(),
        newsletter: Some("mail de prueba".to_string()),
    };

    // Crear algunas ventas
    let venta1 = Venta::crear_venta(
        "12/12/2020".to_string(),
        vec![producto1.clone()],
        cliente.clone(),
        vendedor1.clone(),
        MedioPago::Efectivo,
    );
    let venta2 = Venta::crear_venta(
        "13/12/2020".to_string(),
        vec![producto2.clone()],
        cliente.clone(),
        vendedor1.clone(),
        MedioPago::Tarjeta,
    );
    let venta3 = Venta::crear_venta(
        "14/12/2020".to_string(),
        vec![producto3.clone()],
        cliente.clone(),
        vendedor2.clone(),
        MedioPago::Debito,
    );

    // Crear el sistema de ventas
    let mut sistema_ventas = SisteamaVentas::new(vec![venta1, venta2, venta3]);

    // Obtener el control de ventas por vendedor
    let control = sistema_ventas.control_ventas_vendedor();

    // Verificar los resultados
    let mut expected_control = HashMap::new();
    expected_control.insert("1234".to_string(), 2);
    expected_control.insert("5678".to_string(), 1);

    assert_eq!(control, expected_control);
}

#[test]
fn test_control_ventas_categ() {
    // Crear algunos productos
    let producto1 = Producto::new("Producto 1".to_string(), Categoria::Alimentos, 100.0);
    let producto2 = Producto::new("Producto 2".to_string(), Categoria::Bebidas, 50.0);
    let producto3 = Producto::new("Producto 3".to_string(), Categoria::Tecnologia, 200.0);
    let producto4 = Producto::new("Producto 4".to_string(), Categoria::Limpieza, 30.0);
    let producto5 = Producto::new("Producto 5".to_string(), Categoria::Ropa, 75.0);

    // Crear un vendedor
    let vendedor = Vendedor::new("1234".to_string(), 5, 10000.0);

    // Crear un cliente
    let cliente = Cliente {
        nombre: "Juan".to_string(),
        apellido: "Perez".to_string(),
        direccion: "Av. Siempre Viva 123".to_string(),
        dni: "12345678".to_string(),
        newsletter: Some("mail de prueba".to_string()),
    };

    // Crear algunas ventas
    let venta1 = Venta::crear_venta(
        "12/12/2020".to_string(),
        vec![producto1.clone(), producto2.clone()],
        cliente.clone(),
        vendedor.clone(),
        MedioPago::Efectivo,
    );
    let venta2 = Venta::crear_venta(
        "13/12/2020".to_string(),
        vec![producto3.clone(), producto4.clone()],
        cliente.clone(),
        vendedor.clone(),
        MedioPago::Tarjeta,
    );
    let venta3 = Venta::crear_venta(
        "14/12/2020".to_string(),
        vec![producto5.clone()],
        cliente.clone(),
        vendedor.clone(),
        MedioPago::Debito,
    );

    // Crear el sistema de ventas
    let mut sistema_ventas = SisteamaVentas::new(vec![venta1, venta2, venta3]);

    // Obtener el control de ventas por categoría
    let control = sistema_ventas.control_ventas_categ();

    // Verificar los resultados
    let expected_control = vec![1, 1, 1, 1, 1];

    assert_eq!(control, expected_control);
}
