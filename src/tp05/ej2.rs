use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fs, io::Write};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Playlist {
    canciones: VecDeque<Cancion>,
    nombre: String,
}

impl Playlist {
    fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push_back(cancion);
        self.guardar_en_archivo();
    }

    fn eliminar_cancion(&mut self, cancion: Cancion) {
        for i in 0..self.canciones.len() {
            if self.canciones[i] == cancion {
                self.canciones.remove(i);
                break;
            }
        }
        self.guardar_en_archivo();
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
        self.guardar_en_archivo();
    }

    fn modificar_titulo(&mut self, titulo: String) {
        self.nombre = titulo;
        self.guardar_en_archivo();
    }

    fn elminar_canciones(&mut self) {
        self.canciones.clear();
        self.guardar_en_archivo();
    }

    fn guardar_en_archivo(&self) {
        let json = serde_json::to_string(&self).expect("No se pudo serializar la playlist");
        let mut file = fs::File::create("playlist.json").expect("No se pudo crear el archivo");
        file.write_all(json.as_bytes())
            .expect("No se pudo escribir en el archivo");
    }

    fn cargar_de_archivo() -> Playlist {
        let data = fs::read_to_string("playlist.json").expect("No se pudo leer el archivo");
        serde_json::from_str(&data).expect("No se pudo deserializar la playlist")
    }
}

#[test]
fn test_agregar() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    playlist.agregar_cancion(cancion.clone());
    assert_eq!(playlist.canciones[0] == cancion, true);
}

#[test]
fn test_eliminar() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    playlist.agregar_cancion(cancion.clone());
    playlist.eliminar_cancion(cancion.clone());
    assert_eq!(playlist.canciones.len(), 0);
}

#[test]
fn test_mover() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    let cancion2 = Cancion {
        titulo: "Cancion2".to_string(),
        artista: "Artista2".to_string(),
        genero: Genero::Rock,
    };
    playlist.agregar_cancion(cancion.clone());
    playlist.agregar_cancion(cancion2.clone());
    playlist.mover_cancion(cancion.clone(), 1);
    assert_eq!(playlist.canciones[1] == cancion, true);
}

fn test_modificar() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    playlist.modificar_titulo("Nuevo".to_string());
    assert_eq!(playlist.nombre, "Nuevo".to_string());
}

#[test]
fn test_eliminar_canciones() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    playlist.agregar_cancion(cancion.clone());
    playlist.elminar_canciones();
    assert_eq!(playlist.canciones.len(), 0);
}
