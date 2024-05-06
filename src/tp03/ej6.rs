struct Examen {
    nombre: String,
    nota: f64,
}

struct Estudiante {
    nombre: String,
    id: u32,
    examenes: Vec<Examen>,
}

impl Examen {
    fn new(nombre: String, nota: f64) -> Self {
        Examen { nombre, nota }
    }
}

impl Estudiante {
    fn obtener_promedio(&self) -> Option<f64> {
        if self.examenes.is_empty() {
            return None;
        }

        let suma_notas: f64 = self.examenes.iter().map(|examen| examen.nota).sum();
        Some(suma_notas / self.examenes.len() as f64)
    }

    fn obtener_calificacion_mas_alta(&self) -> Option<f64> {
        if self.examenes.is_empty() {
            return None;
        }

        let mut max_nota = self.examenes[0].nota;
        for examen in &self.examenes {
            if examen.nota > max_nota {
                max_nota = examen.nota;
            }
        }
        Some(max_nota)
    }

    // Método para obtener la calificación más baja
    fn obtener_calificacion_mas_baja(&self) -> Option<f64> {
        if self.examenes.is_empty() {
            return None;
        }

        let mut min_nota = self.examenes[0].nota;
        for examen in &self.examenes {
            if examen.nota < min_nota {
                min_nota = examen.nota;
            }
        }
        Some(min_nota)
    }
}
