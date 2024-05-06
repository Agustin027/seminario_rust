use std::collections::VecDeque; // Import the VecDeque type from the collections module

use super::ej3::Fecha;

struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola_de_atencion: VecDeque<Mascota>,
    atenciones_realizadas: Vec<Atencion>,
}
#[derive(Debug, PartialEq, Eq)]
enum TipoAnimal {
    perro,
    gato,
    caballo,
    otros,
}

#[derive(Debug, PartialEq, Eq)]
struct Atencion {
    datos_mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    fecha: Fecha,
}

#[derive(Debug, PartialEq, Eq)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: TipoAnimal,
    dueño: Dueño,
}

#[derive(Debug, PartialEq, Eq)]
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
        atenciones_realizadas: Vec<Atencion>,
    ) -> Self {
        Veterinaria {
            nombre,
            direccion,
            id,
            cola_de_atencion,
            atenciones_realizadas,
        }
    }

    fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola_de_atencion.push_back(mascota)
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
    }
    fn buscar_atencion(&self, nombre_mascota: String, nombre_dueño: String, telefono: u32) {
        for i in 0..self.atenciones_realizadas.len() {
            if self.atenciones_realizadas[i].datos_mascota.nombre == nombre_mascota
                && self.atenciones_realizadas[i].datos_mascota.dueño.nombre == nombre_dueño
                && self.atenciones_realizadas[i].datos_mascota.dueño.telefono == telefono
            {
                println!("Encontre mascota :P");
                break;
            }
        }
    }
    fn modifcar_diagnostico(&mut self, atencion: Atencion, diagnostico: String) {
        for i in 0..self.atenciones_realizadas.len() {
            if self.atenciones_realizadas[i] == atencion {
                self.atenciones_realizadas[i].diagnostico = diagnostico;
                break;
            }
        }
    }
    fn modificar_fecha(&mut self, atencion: Atencion, fecha: Fecha) {
        for i in 0..self.atenciones_realizadas.len() {
            if self.atenciones_realizadas[i] == atencion {
                self.atenciones_realizadas[i].fecha = fecha;
                break;
            }
        }
    }

    fn eliminar_atencion(&mut self, atencion: Atencion) {
        for i in 0..self.atenciones_realizadas.len() {
            if self.atenciones_realizadas[i] == atencion {
                self.atenciones_realizadas.remove(i);
                break;
            }
        }
    }
}
