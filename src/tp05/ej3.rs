//Falta hacer punto B
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    error::Error,
    fmt::{self, Display},
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
};

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

impl From<std::io::Error> for MiError {
    fn from(error: std::io::Error) -> Self {
        MiError {
            msg: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for MiError {
    fn from(error: serde_json::Error) -> Self {
        MiError {
            msg: error.to_string(),
        }
    }
}
use super::fecha::Fecha;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola_de_atencion: VecDeque<Mascota>,
    atenciones_realizadas: Vec<Atencion>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum TipoAnimal {
    perro,
    gato,
    caballo,
    otros,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Atencion {
    datos_mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    fecha: Fecha,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: TipoAnimal,
    dueño: Dueño,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Dueño {
    nombre: String,
    direccion: String,
    telefono: u32,
}

impl Veterinaria {
    fn new(
        nombre: String,
        direccion: String,
        id: u32,
        cola_de_atencion: VecDeque<Mascota>,
    ) -> Self {
        let atenciones_realizadas = match OpenOptions::new().read(true).open("src/tp05/ej3.json") {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                serde_json::from_str(&buf).unwrap()
            }
            Err(_) => Vec::new(),
        };

        Veterinaria {
            nombre,
            direccion,
            id,
            cola_de_atencion,
            atenciones_realizadas,
        }
    }

    fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola_de_atencion.push_back(mascota);
    }
    fn agregar_mascota_prioridad(&mut self, mascota: Mascota) {
        self.cola_de_atencion.push_front(mascota)
    }

    fn atender_mascota(&mut self) {
        self.cola_de_atencion.pop_front();
    }

    fn eliminar_mascota(&mut self, mascota: Mascota) {
        for i in 0..self.cola_de_atencion.len() {
            if self.cola_de_atencion[i] == mascota {
                self.cola_de_atencion.remove(i);
                break;
            }
        }
    }

    fn registrar_atencion(&mut self, atencion: Atencion) {
        self.atenciones_realizadas.push(atencion);
        self.guardar_atencion();
    }
    fn buscar_atencion(
        &self,
        nombre_mascota: String,
        nombre_dueño: String,
        telefono: u32,
    ) -> Option<Atencion> {
        let mut aux = None;
        for i in 0..self.atenciones_realizadas.len() {
            if self.atenciones_realizadas[i].datos_mascota.nombre == nombre_mascota
                && self.atenciones_realizadas[i].datos_mascota.dueño.nombre == nombre_dueño
                && self.atenciones_realizadas[i].datos_mascota.dueño.telefono == telefono
            {
                aux = Some(self.atenciones_realizadas[i].clone());
                return aux;
            }
        }
        return aux;
    }
    fn modifcar_diagnostico(&mut self, atencion: Atencion, diagnostico: String) {
        for i in 0..self.atenciones_realizadas.len() {
            if self.atenciones_realizadas[i] == atencion {
                self.atenciones_realizadas[i].diagnostico = diagnostico;
                break;
            }
        }
        self.guardar_atencion();
    }
    fn modificar_fecha(&mut self, atencion: Atencion, fecha: Fecha) {
        for i in 0..self.atenciones_realizadas.len() {
            if self.atenciones_realizadas[i] == atencion {
                self.atenciones_realizadas[i].fecha = fecha;
                break;
            }
        }
        self.guardar_atencion();
    }

    fn eliminar_atencion(&mut self, atencion: Atencion) {
        for i in 0..self.atenciones_realizadas.len() {
            if self.atenciones_realizadas[i] == atencion {
                self.atenciones_realizadas.remove(i);
                break;
            }
        }
        self.guardar_atencion();
    }

    fn guardar_atencion(&self) -> Result<(), MiError> {
        let file_path = "src/tp05/ej3.json";

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // Esto trunca el archivo a 0 bytes si ya existe
            .open(&file_path)?;

        let serialized = serde_json::to_string(&self.atenciones_realizadas)?;
        file.write_all(serialized.as_bytes());
        Ok(())
    }
}

#[test]
fn constructor() {
    let veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        VecDeque::new(),
    );
    assert_eq!(veterinaria.nombre, "Veterinaria".to_string());
    assert_eq!(veterinaria.direccion, "Direccion".to_string());
    assert_eq!(veterinaria.id, 1);
    assert!(veterinaria.cola_de_atencion.is_empty());
}

#[test]
fn test_agregar_mascota() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        VecDeque::new(),
    );
    let mascota = Mascota {
        nombre: "Mascota".to_string(),
        edad: 1,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño".to_string(),
            direccion: "Direccion".to_string(),
            telefono: 123,
        },
    };
    veterinaria.agregar_mascota(mascota.clone());
    assert_eq!(veterinaria.cola_de_atencion[0] == mascota, true);
}

#[test]
fn test_agregar_mascota_prioridad() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        5, // Capacidad para 5 mascotas
        VecDeque::new(),
    );
    let mascota1 = Mascota {
        nombre: "Mascota1".to_string(),
        edad: 2,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño1".to_string(),
            direccion: "Direccion1".to_string(),
            telefono: 1111111,
        },
    };
    let mascota2 = Mascota {
        nombre: "Mascota2".to_string(),
        edad: 3,
        tipo: TipoAnimal::gato,
        dueño: Dueño {
            nombre: "Dueño2".to_string(),
            direccion: "Direccion2".to_string(),
            telefono: 2222222,
        },
    };
    let mascota3 = Mascota {
        nombre: "Mascota3".to_string(),
        edad: 4,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño3".to_string(),
            direccion: "Direccion3".to_string(),
            telefono: 3333333,
        },
    };
    veterinaria.agregar_mascota(mascota1.clone());
    veterinaria.agregar_mascota(mascota2.clone());
    veterinaria.agregar_mascota_prioridad(mascota3.clone());

    assert_eq!(veterinaria.cola_de_atencion[0] == mascota3, true);
}

#[test]
fn test_atender_mascota() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        VecDeque::new(),
    );
    let mascota = Mascota {
        nombre: "Mascota".to_string(),
        edad: 1,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño".to_string(),
            direccion: "Direccion".to_string(),
            telefono: 123,
        },
    };
    veterinaria.agregar_mascota(mascota.clone());
    veterinaria.atender_mascota();
    assert_eq!(veterinaria.cola_de_atencion.is_empty(), true);
}

#[test]
fn test_eliminar_mascota() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        VecDeque::new(),
    );
    let mascota1 = Mascota {
        nombre: "Mascota1".to_string(),
        edad: 2,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño1".to_string(),
            direccion: "Direccion1".to_string(),
            telefono: 1111111,
        },
    };
    let mascota2 = Mascota {
        nombre: "Mascota2".to_string(),
        edad: 3,
        tipo: TipoAnimal::gato,
        dueño: Dueño {
            nombre: "Dueño2".to_string(),
            direccion: "Direccion2".to_string(),
            telefono: 2222222,
        },
    };
    let mascota3 = Mascota {
        nombre: "Mascota3".to_string(),
        edad: 4,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño3".to_string(),
            direccion: "Direccion3".to_string(),
            telefono: 3333333,
        },
    };
    veterinaria.agregar_mascota(mascota1.clone());
    veterinaria.agregar_mascota(mascota2.clone());
    veterinaria.agregar_mascota(mascota3.clone());
    veterinaria.eliminar_mascota(mascota2.clone());
    assert!(veterinaria.cola_de_atencion.len() == 2);
}

#[test]
fn test_registrar_atencion() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        VecDeque::new(),
    );
    let mascota = Mascota {
        nombre: "Mascota".to_string(),
        edad: 1,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño".to_string(),
            direccion: "Direccion".to_string(),
            telefono: 123,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Fecha::new(1, 1, 2021),
    };
    veterinaria.atenciones_realizadas.clear();
    veterinaria.registrar_atencion(atencion.clone());
    veterinaria.registrar_atencion(atencion.clone());
    assert_eq!(veterinaria.atenciones_realizadas[0] == atencion, true);
}

#[test]
fn test_buscar_atencion() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        VecDeque::new(),
    );
    let mascota = Mascota {
        nombre: "Mascota".to_string(),
        edad: 1,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño".to_string(),
            direccion: "Direccion".to_string(),
            telefono: 123,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Fecha::new(1, 1, 2021),
    };
    veterinaria.registrar_atencion(atencion.clone());

    let encontre = veterinaria
        .buscar_atencion("Mascota".to_string(), "Dueño".to_string(), 123)
        .is_some();
    let no_encontre = veterinaria
        .buscar_atencion("Mascota".to_string(), "Messi".to_string(), 123)
        .is_none();
    assert_eq!(encontre, true);
    assert_eq!(no_encontre, true);
}

#[test]
fn test_modificar_diagnostico() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        VecDeque::new(),
    );
    let mascota = Mascota {
        nombre: "Mascota".to_string(),
        edad: 1,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño".to_string(),
            direccion: "Direccion".to_string(),
            telefono: 123,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Fecha::new(1, 1, 2021),
    };
    veterinaria.registrar_atencion(atencion.clone());
    veterinaria.modifcar_diagnostico(atencion.clone(), "Nuevo Diagnostico".to_string());
    assert_eq!(
        veterinaria.atenciones_realizadas[0].diagnostico,
        "Nuevo Diagnostico".to_string()
    );
}

#[test]
fn test_modificar_fecha() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        VecDeque::new(),
    );
    let mascota = Mascota {
        nombre: "Mascota".to_string(),
        edad: 1,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño".to_string(),
            direccion: "Direccion".to_string(),
            telefono: 123,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Fecha::new(1, 1, 2024),
    };
    veterinaria.atenciones_realizadas.clear();
    veterinaria.registrar_atencion(atencion.clone());
    veterinaria.modificar_fecha(atencion.clone(), Fecha::new(2, 2, 2024));
    assert_eq!(
        veterinaria.atenciones_realizadas[0]
            .fecha
            .equals(&Fecha::new(2, 2, 2024)),
        true
    );
}

#[test]
fn test_eliminar() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        VecDeque::new(),
    );
    let mascota = Mascota {
        nombre: "Mascota".to_string(),
        edad: 1,
        tipo: TipoAnimal::perro,
        dueño: Dueño {
            nombre: "Dueño".to_string(),
            direccion: "Direccion".to_string(),
            telefono: 123,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Fecha::new(1, 1, 2024),
    };
    veterinaria.atenciones_realizadas.clear();
    veterinaria.registrar_atencion(atencion.clone());
    veterinaria.eliminar_atencion(atencion.clone());
    assert!(veterinaria.atenciones_realizadas.is_empty());
}
