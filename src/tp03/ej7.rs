struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    cap: i32,
    autos: Vec<Auto>,
}
struct Auto {
    marca: String,
    modelo: String,
    año: u32,
    preciob: f64,
    color: String,
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

    fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() < self.cap as usize {
            self.autos.push(auto);
            return true;
        }
        return false;
    }
    fn eliminar_auto(&mut self, auto: Auto) {
        for i in 0..self.autos.len() {
            let aux = &self.autos[i];
            if aux.año == auto.año
                && aux.marca == auto.marca
                && aux.preciob == auto.preciob
                && aux.modelo == auto.modelo
                && aux.color == auto.color
            {
                self.autos.remove(i);
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
                && aux.color == auto.color
            {
                return Some(aux);
            }
        }
        None
    }
}

impl Auto {
    fn new(marca: String, modelo: String, año: u32, preciob: f64, color: String) -> Auto {
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
        if self.color == "rojo" || self.color == "azul" || self.color == "amarillo" {
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
