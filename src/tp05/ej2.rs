use std::{clone, collections::VecDeque};
#[derive(Debug, Clone)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}
impl Cancion {
    pub fn equals(&self, cancion: &Cancion) -> bool {
        self.titulo == cancion.titulo
            && self.artista == cancion.artista
            && self.genero.equals(&cancion.genero)
    }
}

#[derive(Debug, Clone)]
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}
impl Genero {
    pub fn genero_to_string(&self) -> String {
        match self {
            Self::Rock => "Rock".to_string(),
            Self::Pop => "Pop".to_string(),
            Self::Rap => "Rap".to_string(),
            Self::Jazz => "Jazz".to_string(),
            Self::Otros => "Otros".to_string(),
        }
    }

    pub fn equals(&self, genero: &Genero) -> bool {
        self.genero_to_string() == genero.genero_to_string()
    }
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
            if self.canciones[i].equals(&cancion) {
                self.canciones.remove(i);
                break;
            }
        }
    }

    fn mover_cancion(&mut self, cancion: Cancion, j: usize) {
        for i in 0..self.canciones.len() {
            if self.canciones[i].equals(&cancion) {
                if let Some(aux) = self.canciones.get(i).cloned() {
                    self.canciones.remove(i);
                    self.canciones.insert(j, aux);
                }
            }
        }
    }
    fn buscar_cancion(&self, cancion: Cancion) -> Option<Cancion> {
        let mut aux = None;
        for i in 0..self.canciones.len() {
            if self.canciones[i].equals(&cancion) {
                aux = Some(self.canciones[i].clone());
                return aux;
            }
        }
        return aux;
    }

    fn obtener_canciones_genero(&self, genero: Genero) -> VecDeque<Cancion> {
        let mut aux: VecDeque<Cancion> = VecDeque::new();
        for i in 0..self.canciones.len() {
            if self.canciones[i].genero.equals(&genero) {
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
    assert_eq!(playlist.canciones[0].equals(&cancion), true);
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
    assert_eq!(playlist.canciones[1].equals(&cancion), true);
}

#[test]
fn test_buscar() {
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
    let encontre = playlist.buscar_cancion(cancion.clone()).is_some();
    let no_encontre = playlist.buscar_cancion(cancion2.clone()).is_none();
    assert_eq!(encontre, true);
    assert_eq!(no_encontre, true);
}

#[test]
fn test_genero() {
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
        genero: Genero::Pop,
    };

    playlist.agregar_cancion(cancion.clone());
    playlist.agregar_cancion(cancion2.clone());
    let canciones = playlist.obtener_canciones_genero(Genero::Rock);
    assert_eq!(canciones[0].equals(&cancion), true);
}

#[test]
fn test_artista() {
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
        genero: Genero::Pop,
    };

    playlist.agregar_cancion(cancion.clone());
    playlist.agregar_cancion(cancion2.clone());
    let canciones = playlist.obtener_canciones_artista("Artista".to_string());
    assert_eq!(canciones[0].equals(&cancion), true);
}

#[test]
fn test_modificar() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    playlist.modificar_titulo("Nuevo".to_string());
    assert_eq!(playlist.nombre, "Nuevo".to_string());
}

#[test]
fn test_eliminar_cancione() {
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
