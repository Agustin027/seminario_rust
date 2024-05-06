use std::{clone, collections::VecDeque};
#[derive(Debug, Clone, PartialEq, Eq)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

struct Playlist {
    canciones: VecDeque<Cancion>,
    nombre: String,
}

impl Playlist {
    fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push_back(cancion);
    }

    fn eliminar_cancion(&mut self, cancion: Cancion) {
        for i in 0..self.canciones.len() {
            if self.canciones[i] == cancion {
                self.canciones.remove(i);
                break;
            }
        }
    }

    fn mover_cancion(&mut self, cancion: Cancion, j: usize) {
        for i in 0..self.canciones.len() {
            if self.canciones[i] == cancion {
                if let Some(aux) = self.canciones.get(i).cloned() {
                    self.canciones.remove(i);
                    self.canciones.insert(j, aux);
                }
            }
        }
    }
    fn buscar_cancion(&self, cancion: Cancion) -> bool {
        for i in 0..self.canciones.len() {
            if self.canciones[i] == cancion {
                return true;
            }
        }
        return false;
    }

    fn obtener_canciones_genero(&self, genero: Genero) -> VecDeque<Cancion> {
        let mut aux: VecDeque<Cancion> = VecDeque::new();
        for i in 0..self.canciones.len() {
            if self.canciones[i].genero == genero {
                aux.push_back(self.canciones[i].clone());
            }
        }
        aux
    }

    fn obtener_canciones_artista(&self, artista: String) -> VecDeque<Cancion> {
        let mut aux: VecDeque<Cancion> = VecDeque::new();
        for i in 0..self.canciones.len() {
            if self.canciones[i].artista == artista {
                aux.push_back(self.canciones[i].clone());
            }
        }
        return aux;
    }

    fn modificar_titulo(&mut self, titulo: String) {
        self.nombre = titulo
    }

    fn elminar_canciones(&mut self) {
        self.canciones.clear()
    }
}
