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
fn ej1() {
    let mut persona1 = Persona::new("Lionel".to_string(), 36, Some("Direc".to_string()));

    println!("{} edad {}", persona1.to_string(), persona1.obtener_edad());
    persona1.to_string();
    persona1.actualizar_direccion(Some("Nueva direccion".to_string()));
    println!("{} edad {}", persona1.to_string(), persona1.obtener_edad());
}
