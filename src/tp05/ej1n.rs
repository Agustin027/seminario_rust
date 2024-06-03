use serde::{Deserialize, Serialize};

use std::{
    error::Error,
    fmt::{self, Display},
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
impl Color {
    pub fn color_to_string(&self) -> String {
        match self {
            Self::Rojo => "Rojo".to_string(),
            Self::Verde => "Verde".to_string(),
            Self::Azul => "Azul".to_string(),
            Self::Amarillo => "Amarillo".to_string(),
            Self::Blanco => "Blanco".to_string(),
            Self::Negro => "Negro".to_string(),
            Self::Otro => "Otro".to_string(),
        }
    }

    pub fn equals(&self, color: &Color) -> bool {
        self.color_to_string() == color.color_to_string()
    }
}

struct MiErr;
impl Display for MiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Supera el limite")
    }
}

#[derive(Debug)]
struct MiError {
    msg: String,
}

impl std::error::Error for MiError {}

impl std::fmt::Display for MiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    fn agregar_auto(&mut self, auto: &Auto) -> Result<(), MiErr> {
        if self.autos.len() < self.cap as usize {
            self.autos.push(auto.clone());
            let path = "src/tp05/autos.json";
            let mut archivo = File::create(path).unwrap();
            let autos_serializados = serde_json::to_string(&self.autos).unwrap();

            archivo.write_all(autos_serializados.as_bytes()).unwrap();
            return Ok(());
        } else {
            return Err(MiErr);
        }
    }

    fn eliminar_auto(&mut self, auto: &Auto) {
        for i in 0..self.autos.len() {
            let aux = &self.autos[i];
            if aux.año == auto.año
                && aux.marca == auto.marca
                && aux.preciob == auto.preciob
                && aux.modelo == auto.modelo
                && aux.color.equals(&auto.color)
            {
                self.autos.remove(i);
                let path = "src/tp05/autos.json";
                let mut archivo = File::create(path).unwrap();
                let autos_serializados = serde_json::to_string(&self.autos).unwrap();
                archivo.write_all(autos_serializados.as_bytes()).unwrap();
                break;
            }
        }
    }
    fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        for aux in self.autos.iter() {
            if aux.año == auto.año
                && aux.marca == auto.marca
                && aux.preciob == auto.preciob
                && aux.modelo == auto.modelo
                && aux.color.equals(&auto.color)
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
        if self.color.color_to_string() == "Rojo"
            || self.color.color_to_string() == "Azul"
            || self.color.color_to_string() == "Amarillo"
        {
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
    assert_eq!(auto.color.color_to_string(), "Rojo".to_string());
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
    assert_eq!(auto.precio_total().round(), 13656.0);
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
        3,
        vec![],
    );
    concesionario.agregar_auto(&auto);
    concesionario.agregar_auto(&auto);
    assert!(concesionario.agregar_auto(&auto).is_ok());
    assert!(concesionario.agregar_auto(&auto).is_err());
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
    let mut concesionario = ConcesionarioAuto::new(
        "Concesionario".to_string(),
        "Direccion".to_string(),
        2,
        vec![],
    );
    concesionario.agregar_auto(&auto);
    concesionario.agregar_auto(&auto2);
    concesionario.eliminar_auto(&auto2);
    assert_eq!(concesionario.autos.len(), 1);
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
    let encontre: bool = concesionario.buscar_auto(&auto).is_some();
    let no_encontre: bool = concesionario.buscar_auto(&auto3).is_none();
    assert_eq!(encontre, true);
    assert_eq!(no_encontre, true);
}
