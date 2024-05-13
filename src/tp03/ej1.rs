struct Persona {
    nombre: String,
    edad: u32,
    direccion: Option<String>,
}

impl Persona {
    // Constructor
    fn new(nombre: String, edad: u32, direccion: Option<String>) -> Self {
        Persona {
            nombre,
            edad,
            direccion,
        }
    }

    // Método para obtener la representación de la persona como String
    fn to_string(&self) -> String {
        let mut result = format!("Nombre: {}, Edad: {}", self.nombre, self.edad);
        if let Some(direccion) = &self.direccion {
            result.push_str(&format!(", Dirección: {}", direccion));
        }
        result
    }

    // Método para obtener la edad de la persona
    fn obtener_edad(&self) -> u32 {
        self.edad
    }

    // Método para actualizar la dirección de la persona
    fn actualizar_direccion(&mut self, nueva_direccion: Option<String>) {
        self.direccion = nueva_direccion;
    }
}

#[test]

fn constructor() {
    let persona = Persona::new("Messi".to_string(), 36, Some("91 Compass Lane".to_string()));
    assert!(persona.nombre == "Messi".to_string());
    assert!(persona.edad == 36);
    assert!(persona.direccion == Some("91 Compass Lane".to_string()));
}

#[test]
fn to_string() {
    let persona = Persona::new("Messi".to_string(), 36, Some("91 Compass Lane".to_string()));
    assert!(
        persona.to_string() == "Nombre: Messi, Edad: 36, Dirección: 91 Compass Lane".to_string()
    );
}
#[test]
fn edad() {
    let persona = Persona::new("Messi".to_string(), 36, Some("91 Compass Lane".to_string()));
    assert!(persona.obtener_edad() == 36);
}

#[test]
fn actualizar_direccion() {
    let mut persona = Persona::new("Messi".to_string(), 36, Some("91 Compass Lane".to_string()));
    persona.actualizar_direccion(Some("Estado de Israel 525".to_string()));
    assert!(persona.direccion == Some("Estado de Israel 525".to_string()));
}

#[test]

fn sin_direccion() {
    let persona = Persona::new("Messi".to_string(), 36, None);
    assert!(persona.direccion == None);
}

#[test]
fn actualizar_direccion_none() {
    let mut persona = Persona::new("Messi".to_string(), 36, Some("91 Compass Lane".to_string()));
    persona.actualizar_direccion(None);
    assert!(persona.direccion == None);
}
