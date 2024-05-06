use super::ej3::Fecha;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(PartialEq, Eq, Hash)]
struct Libro {
    titulo: String,
    autor: String,
    numero_de_paginas: u32,
    genero: Genero,
}

#[derive(PartialEq, Eq, Hash)]
struct Cliente {
    nombre: String,
    telefono: u32,
    correo: String,
}
#[derive(PartialEq, Eq, Hash)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: bool, // True si fue devuelto, False si no
}

impl Prestamo {
    fn new(libro: Libro, cliente: Cliente, fecha_vencimiento: Fecha) -> Prestamo {
        Prestamo {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion: None,
            estado: false, // Se inicializa como no devuelto
        }
    }
}

struct Biblioteca {
    nombre: String,
    direccion: String,
    libros_disponibles: HashMap<Libro, i32>,
    prestamos: Vec<Prestamo>,
}

impl Biblioteca {
    fn obtener_cant_copias(&self, libro: &Libro) -> i32 {
        *self.libros_disponibles.get(libro).unwrap_or(&0)
    }

    fn decrementar_copias(&mut self, libro: &Libro) {
        if let Some(cant) = self.libros_disponibles.get_mut(libro) {
            if *cant > 0 {
                *cant -= 1;
            }
        }
    }

    fn incrementar_copias(&mut self, libro: &Libro) {
        if let Some(cant) = self.libros_disponibles.get_mut(libro) {
            *cant += 1;
        }
    }

    fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let mut contador = 0;
        for prestamo in &self.prestamos {
            if prestamo.cliente == *cliente && prestamo.estado == true {
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
        // Verificar si el cliente tiene menos de 5 préstamos en estado "en préstamo"
        if self.contar_prestamos_cliente(&cliente) >= 5 {
            return false;
        }

        // Verificar si hay al menos una copia disponible del libro
        if self.obtener_cant_copias(&libro) < 1 {
            return false;
        }

        // Realizar el préstamo
        self.decrementar_copias(&libro);
        let prestamo = Prestamo::new(libro, cliente, fecha_vencimiento);
        self.prestamos.push(prestamo);

        true
    }
    fn prestamos_a_vencer(&self, dias: u32) /*-> Vec<Prestamo>\*/
    {
        // falta implementar
    }
    fn prestamos_vencidos(&self) /*-> Vec<Prestamo>\*/
    {
        // falta implementar
    }
    fn devolver_libro(&mut self, libro: Libro, cliente: Cliente) -> bool {
        // falta implementar
        false
    }
}
