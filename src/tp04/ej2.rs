trait EsMayor {
    fn es_mayor_salario(&self, num: f64) -> bool;
    fn es_mayor_edad(&self, num: u8) -> bool;
}

impl<'a> EsMayor for Persona<'a> {
    fn es_mayor_salario(&self, num: f64) -> bool {
        self.salario > num
    }

    fn es_mayor_edad(&self, num: u8) -> bool {
        self.edad > num
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

fn listado_salario_mayor<'a>(vec: &'a Vec<Persona<'a>>, num: f64) -> Vec<&'a Persona<'a>> {
    vec.iter().filter(|&x| x.es_mayor_salario(num)).collect()
}

fn persona_en_ciudad<'a>(
    vec: &'a Vec<Persona<'a>>,
    edad: u8,
    ciudad: &str,
) -> Vec<&'a Persona<'a>> {
    let iter = vec.iter();

    iter.filter(|&x| x.es_mayor_edad(edad) && x.ciudad == ciudad)
        .collect()
}

fn todas_en_ciudad<'a>(vec: &'a Vec<Persona<'a>>, ciudad: &str) -> bool {
    vec.iter().all(|x| x.ciudad == ciudad)
}

fn almenos_uno_en_ciudad<'a>(vec: &'a Vec<Persona<'a>>, ciudad: &str) -> bool {
    vec.iter().any(|x| x.ciudad == ciudad)
}

fn existe<'a>(vec: &'a Vec<Persona<'a>>, p: Persona) -> bool {
    vec.contains(&p)
}

fn edades(personas: &Vec<Persona>) -> Vec<u8> {
    personas.iter().map(|persona| persona.edad).collect()
}

// despues volver a hacer xD
fn retornar_max_y_min_salario<'a>(
    personas: &'a Vec<Persona<'a>>,
) -> (&'a Persona<'a>, &'a Persona<'a>) {
    let (max_persona, min_persona) = personas.iter().skip(1).fold(
        (&personas[0], &personas[0]),
        |(max_so_far, min_so_far), persona| {
            let max_so_far = if persona.salario > max_so_far.salario
                || (persona.salario == max_so_far.salario && persona.edad > max_so_far.edad)
            {
                persona
            } else {
                max_so_far
            };

            let min_so_far = if persona.salario < min_so_far.salario
                || (persona.salario == min_so_far.salario && persona.edad > min_so_far.edad)
            {
                persona
            } else {
                min_so_far
            };

            (max_so_far, min_so_far)
        },
    );

    (max_persona, min_persona)
}

#[test]
fn test_listado_salario_mayor() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(listado_salario_mayor(&vec, 1500.0), vec![&p2, &p3]);
}

#[test]
fn test_persona_en_ciudad() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(persona_en_ciudad(&vec, 35, "Springfield"), vec![&p2, &p3]);
}

#[test]
fn test_todas_en_ciudad() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(todas_en_ciudad(&vec, "Springfield"), true);
}

#[test]
fn test_almenos_uno_en_ciudad() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(almenos_uno_en_ciudad(&vec, "Springfield"), true);
}
#[test]
fn test_existe() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(existe(&vec, p1), true);
}

#[test]
fn test_edades() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(edades(&vec), vec![30, 40, 50]);
}

#[test]
fn test_retornar_max_y_min_salario() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let p4 = Persona {
        nombre: "Marge",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 60,
    };
    let vec = vec![p1.clone(), p2, p3, p4.clone()];
    assert_eq!(retornar_max_y_min_salario(&vec), (&p4, &p1));
}
