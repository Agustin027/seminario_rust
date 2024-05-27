use super::fecha::Fecha;
use std::{collections::HashMap, f32::consts::E};

#[derive(Debug, Clone, PartialEq)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Debug, Clone, PartialEq)]
struct Libro {
    isbn: u32,
    titulo: String,
    autor: String,
    numero_de_paginas: u32,
    genero: Genero,
}

#[derive(Debug, Clone, PartialEq)]
struct Cliente {
    nombre: String,
    telefono: u32,
    correo: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: bool,
}

impl Prestamo {
    fn new(libro: Libro, cliente: Cliente, fecha_vencimiento: Fecha) -> Prestamo {
        Prestamo {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion: None,
            estado: false,
        }
    }
}

struct Biblioteca {
    nombre: String,
    direccion: String,
    libros_disponibles: HashMap<u32, i32>,
    prestamos: Vec<Prestamo>,
}

impl Biblioteca {
    fn obtener_cant_copias(&self, libro: &Libro) -> i32 {
        *self.libros_disponibles.get(&libro.isbn).unwrap_or(&0)
    }

    fn decrementar_copias(&mut self, libro: &Libro) {
        if let Some(cant) = self.libros_disponibles.get_mut(&libro.isbn) {
            if *cant > 0 {
                *cant -= 1;
            }
        }
    }

    fn incrementar_copias(&mut self, libro: &Libro) {
        if let Some(cant) = self.libros_disponibles.get_mut(&libro.isbn) {
            *cant += 1;
        }
    }

    fn contar_prestamos_cliente(&self, cliente: Cliente) -> u32 {
        let mut contador = 0;
        for prestamo in &self.prestamos {
            if prestamo.cliente == cliente && prestamo.estado == true {
                contador += 1;
            }
        }
        contador
    }

    fn realizar_prestamo(
        &mut self,
        libro: Libro,
        cliente: Cliente,
        fecha_vencimiento: Fecha,
    ) -> bool {
        if self.contar_prestamos_cliente(cliente.clone()) >= 5
            && self.obtener_cant_copias(&libro) < 1
        {
            return false;
        } else {
            self.decrementar_copias(&libro);
            let mut prestamo = Prestamo::new(libro, cliente, fecha_vencimiento);
            prestamo.estado = true;
            self.prestamos.push(prestamo);
            return true;
        }
    }

    fn prestamos_a_vencer(&self, fecha: Fecha) -> Vec<Prestamo> {
        let mut prestamos = Vec::new();
        for i in 0..self.prestamos.len() {
            if self.prestamos[i].fecha_vencimiento.es_mayor(fecha.clone()) {
                prestamos.push(self.prestamos[i].clone());
            }
        }
        return prestamos;
    }
    fn prestamos_vencidos(&self, fecha: Fecha) -> Vec<Prestamo> {
        let mut prestamos = Vec::new();
        for i in 0..self.prestamos.len() {
            if fecha.es_mayor(self.prestamos[i].fecha_vencimiento.clone()) {
                prestamos.push(self.prestamos[i].clone());
            }
        }
        return prestamos;
    }
    fn devolver_libro(&mut self, libro: Libro, cliente: Cliente, fecha: Fecha) -> bool {
        for i in 0..self.prestamos.len() {
            if self.prestamos[i].libro == libro && self.prestamos[i].cliente == cliente {
                self.prestamos[i].estado = false;
                self.prestamos[i].fecha_devolucion = Some(fecha);
                self.incrementar_copias(&libro);
                return true;
            }
        }
        false
    }
}

#[test]
fn test_constructor_prestamo() {
    let libro = Libro {
        isbn: 123,
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };
    let fecha = Fecha::new(7, 6, 2024);
    let prestamo = Prestamo::new(libro.clone(), cliente.clone(), fecha.clone());
    assert_eq!((prestamo.libro == libro), true);
    assert_eq!(prestamo.cliente == cliente, true);
    assert_eq!(prestamo.fecha_vencimiento.equals(&fecha), true);
    assert_eq!(prestamo.fecha_devolucion.is_none(), true);
    assert_eq!(prestamo.estado, false);
}

#[test]
fn test_obtener_cant_copias() {
    let libro = Libro {
        isbn: 123,
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: HashMap::new(),
        prestamos: Vec::new(),
    };
    biblioteca.libros_disponibles.insert(libro.isbn, 5);
    assert_eq!(biblioteca.obtener_cant_copias(&libro), 5);
}

#[test]
fn test_decrementar_copias() {
    let libro = Libro {
        isbn: 123,
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: HashMap::new(),
        prestamos: Vec::new(),
    };
    biblioteca.libros_disponibles.insert(libro.isbn, 5);
    biblioteca.decrementar_copias(&libro);
    assert_eq!(biblioteca.obtener_cant_copias(&libro), 4);
}

#[test]
fn test_incrementar_copias() {
    let libro = Libro {
        isbn: 123,
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: HashMap::new(),
        prestamos: Vec::new(),
    };
    biblioteca.libros_disponibles.insert(libro.isbn, 5);
    biblioteca.incrementar_copias(&libro);
    assert_eq!(biblioteca.obtener_cant_copias(&libro), 6);
}
#[test]
fn test_contar_prestamos_cliente() {
    let libro = Libro {
        isbn: 123,
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };

    let fecha = Fecha::new(7, 6, 2024);

    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: HashMap::new(),
        prestamos: Vec::new(),
    };
    biblioteca.libros_disponibles.insert(libro.isbn, 10);
    // Agregamos algunos préstamos de prueba
    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha.clone());
    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha.clone());
    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha.clone());

    // Comprobamos que la función contar_prestamos_cliente devuelva el recuento correcto
    assert_eq!(biblioteca.contar_prestamos_cliente(cliente), 3);
}

#[test]
fn test_realizar_prestamo() {
    let libro = Libro {
        isbn: 123,
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };

    let fecha = Fecha::new(1, 1, 2021);

    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: HashMap::new(),
        prestamos: Vec::new(),
    };
    biblioteca.libros_disponibles.insert(libro.isbn, 10);
    assert_eq!(
        biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha.clone()),
        true
    );
    assert_eq!(biblioteca.obtener_cant_copias(&libro), 9);
    assert_eq!(biblioteca.prestamos.len(), 1);
}

#[test]
fn test_prestamos_a_vencer() {
    let libro = Libro {
        isbn: 123,
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };

    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: HashMap::new(),
        prestamos: Vec::new(),
    };

    biblioteca.libros_disponibles.insert(libro.isbn, 10);
    let fecha = Fecha::new(1, 1, 2024);
    let fecha_vencimiento = Fecha::new(1, 2, 2024);

    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());

    let prestamos = biblioteca.prestamos_a_vencer(fecha);
    assert_eq!(prestamos.len(), 1);
    assert_eq!(prestamos[0].libro == libro, true);
}

#[test]
fn test_prestamo_vencidos() {
    let libro = Libro {
        isbn: 123,
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };

    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: HashMap::new(),
        prestamos: Vec::new(),
    };

    biblioteca.libros_disponibles.insert(libro.isbn, 10);

    let fecha = Fecha::new(7, 6, 2024);
    let fecha_vencimiento = Fecha::new(7, 7, 2024);
    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());

    let prestamos = biblioteca.prestamos_vencidos(fecha);
    assert_eq!(prestamos.len(), 0);

    let fecha = Fecha::new(7, 8, 2024);
    let prestamos = biblioteca.prestamos_vencidos(fecha);

    assert_eq!(prestamos.len(), 1);
    assert_eq!(prestamos[0].libro == libro, true);
}

#[test]
fn test_devolver_libro() {
    let mut libro = Libro {
        isbn: 123,
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    let mut libro2 = Libro {
        isbn: 456,
        titulo: "Otro Titulo".to_string(),
        autor: "Otro Autor".to_string(),
        numero_de_paginas: 200,
        genero: Genero::Infantil,
    };

    let mut libro3 = Libro {
        isbn: 789,
        titulo: "Tercer Titulo".to_string(),
        autor: "Tercer Autor".to_string(),
        numero_de_paginas: 150,
        genero: Genero::Tecnico,
    };

    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: HashMap::new(),
        prestamos: Vec::new(),
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };

    biblioteca.libros_disponibles.insert(libro.isbn, 10);
    biblioteca.libros_disponibles.insert(libro2.isbn, 8);
    biblioteca.libros_disponibles.insert(libro3.isbn, 12);

    let fecha = Fecha::new(1, 1, 2024);
    let fecha_vencimiento = Fecha::new(1, 2, 2024);

    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente.clone(), fecha_vencimiento.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente.clone(), fecha_vencimiento.clone());

    assert_eq!(
        biblioteca.devolver_libro(libro.clone(), cliente.clone(), fecha.clone()),
        true
    );
    assert_eq!(biblioteca.obtener_cant_copias(&libro), 10);
    assert_eq!(
        biblioteca.devolver_libro(libro2.clone(), cliente.clone(), fecha.clone()),
        true
    );
    assert_eq!(biblioteca.obtener_cant_copias(&libro2), 8);
    assert_eq!(
        biblioteca.devolver_libro(libro3.clone(), cliente.clone(), fecha.clone()),
        true
    );
    assert_eq!(biblioteca.obtener_cant_copias(&libro3), 12);

    assert_eq!(biblioteca.prestamos[0].estado, false);
    assert_eq!(biblioteca.prestamos[1].estado, false);
    assert_eq!(biblioteca.prestamos[2].estado, false);
}
