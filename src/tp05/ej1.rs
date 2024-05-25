use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::Write,
    result,
};

#[derive(Debug, Serialize, Deserialize)]
struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    cap: i32,
    autos: Vec<Auto>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Auto {
    marca: String,
    modelo: String,
    año: u32,
    preciob: f64,
    color: Color,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
    Otro,
}

use std::error::Error;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MiError {
    msg: String,
}

impl Error for MiError {}

impl fmt::Display for MiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl ConcesionarioAuto {
    fn new(nombre: String, direccion: String, cap: i32, autos: Vec<Auto>) -> Self {
        ConcesionarioAuto {
            nombre,
            direccion,
            cap,
            autos,
        }
    }

    fn agregar_auto(&mut self, auto: &Auto) -> Result<(), Box<dyn std::error::Error>> {
        let path = "src/tp05/archivo.json";
        let mut archivo = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;

        if self.autos.len() < self.cap as usize {
            self.autos.push(auto.clone());
            let auto_serializado = serde_json::to_string(&auto)?;
            archivo.write_all(auto_serializado.as_bytes())?; // ¡Agregamos esta línea!
            println!("Auto agregado");
            return Ok(());
        }

        Err(Box::new(MiError {
            msg: "Capacidad máxima alcanzada".to_string(),
        }))
    }

    fn eliminar_auto(&mut self, auto: &Auto) -> Result<(), Box<dyn std::error::Error>> {
        let path = "src/tp05/archivo.json";
        let mut archivo = OpenOptions::new().read(true).write(true).open(path)?;

        let mut autos_actualizados = self.autos.clone(); // Clonamos la lista para modificarla

        for i in 0..autos_actualizados.len() {
            let aux = &autos_actualizados[i];
            if aux.año == auto.año
                && aux.marca == auto.marca
                && aux.preciob == auto.preciob
                && aux.modelo == auto.modelo
                && aux.color == auto.color
            {
                autos_actualizados.remove(i); // Eliminamos el auto de la lista
                let autos_serializados = serde_json::to_string(&autos_actualizados)?;
                archivo.set_len(0)?; // Vaciamos el archivo
                archivo.write_all(autos_serializados.as_bytes())?; // Escribimos la lista actualizada
                println!("Auto eliminado");
                return Ok(());
            }
        }

        Err(Box::new(MiError {
            msg: "Auto no encontrado".to_string(),
        }))
    }
    fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        for aux in self.autos.iter() {
            if aux.año == auto.año
                && aux.marca == auto.marca
                && aux.preciob == auto.preciob
                && aux.modelo == auto.modelo
                && aux.color == auto.color
            {
                return Some(aux);
            }
        }

        None
    }
}

impl Auto {
    fn new(marca: String, modelo: String, año: u32, preciob: f64, color: Color) -> Auto {
        Auto {
            marca,
            modelo,
            año,
            preciob,
            color,
        }
    }
    fn precio_total(&self) -> f64 {
        let mut precio_total = self.preciob;

        // Aplicar recargo o descuento basado en el color
        if self.color == Color::Rojo || self.color == Color::Verde || self.color == Color::Azul {
            precio_total *= 1.25; // Aplicar recargo del 25% si es color primario
        } else {
            precio_total *= 0.9; // Aplicar descuento del 10% si no es color primario
        }

        // Aplicar recargo basado en la marca
        if self.marca == "BMW" {
            precio_total *= 1.15; // Aplicar recargo del 15%
        }

        // Aplicar descuento basado en el año
        if self.año < 2000 {
            precio_total *= 0.95; // Aplicar descuento del 5%
        }

        precio_total
    }
}

#[test]
fn constructor() {
    let auto = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Rojo,
    );
    assert_eq!(auto.marca, "BMW".to_string());
    assert_eq!(auto.modelo, "Serie 3".to_string());
    assert_eq!(auto.año, 1999);
    assert_eq!(auto.preciob, 10000.0);
    assert_eq!(auto.color, Color::Rojo);
}

#[test]
fn total() {
    let auto = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Rojo,
    );
    let auto2 = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Otro,
    );
    assert_eq!(auto.precio_total().round(), 13656.0);
    //revisar
    assert_eq!(auto2.precio_total().round(), 9833.0);
}

#[test]
fn agregar_auto() {
    let auto = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Rojo,
    );

    let mut concesionario = ConcesionarioAuto::new(
        "Concesionario".to_string(),
        "Direccion".to_string(),
        1,
        vec![],
    );
    let res = concesionario.agregar_auto(&auto);
    assert_eq!(res.is_ok(), true);
    let res = concesionario.agregar_auto(&auto);
    assert_eq!(res.is_err(), true);
}

#[test]
fn eliminar_auto() {
    let auto = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Rojo,
    );
    let auto2 = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Verde,
    );
    let auto3 = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1899,
        10000.0,
        Color::Negro,
    );
    let mut concesionario = ConcesionarioAuto::new(
        "Concesionario".to_string(),
        "Direccion".to_string(),
        2,
        vec![],
    );
    concesionario.agregar_auto(&auto);
    concesionario.agregar_auto(&auto2);

    let mut resul = concesionario.eliminar_auto(&auto2);
    assert_eq!(resul.is_ok(), true);
    resul = concesionario.eliminar_auto(&auto3);
    assert_eq!(resul.is_err(), true);
    //assert_eq!(concesionario.autos.len(), 1);
}

#[test]
fn buscar_auto() {
    let auto = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Rojo,
    );
    let auto2 = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Verde,
    );
    let auto3 = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Azul,
    );
    let auto4 = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Amarillo,
    );

    let auto5 = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Blanco,
    );

    let auto6 = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Negro,
    );

    let auto7 = Auto::new(
        "BMW".to_string(),
        "Serie 3".to_string(),
        1999,
        10000.0,
        Color::Otro,
    );

    let mut concesionario = ConcesionarioAuto::new(
        "Concesionario".to_string(),
        "Direccion".to_string(),
        10,
        vec![],
    );

    concesionario.agregar_auto(&auto);
    concesionario.agregar_auto(&auto2);
    concesionario.agregar_auto(&auto3);
    concesionario.agregar_auto(&auto4);
    concesionario.agregar_auto(&auto5);
    concesionario.agregar_auto(&auto6);

    let encontre: bool = concesionario.buscar_auto(&auto).is_some();
    let no_encontre: bool = concesionario.buscar_auto(&auto7).is_none();
    assert_eq!(encontre, true);
    assert_eq!(no_encontre, true);
}
