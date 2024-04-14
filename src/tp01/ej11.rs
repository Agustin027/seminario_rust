use std::io;

fn main() {
    // Definir un arreglo de 5 cadenas
   let arreglo = ["Goku", "Vegeta", "Gohan", "Trunks", "Messi"];

    // Pedir al usuario que ingrese una cadena por teclado
    println!("Ingrese una cadena:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada.");

    // Verificar si la cadena ingresada por el usuario se encuentra en el arreglo
    let cadena_ingresada = input.trim();

    let mut encontrado = false;
    for i in 0..5 {
        if arreglo[i] == cadena_ingresada {
            encontrado = true;

        }
    }

    // Imprimir un mensaje si la cadena se encuentra en el arreglo
    if encontrado {
        println!("La cadena ingresada se encuentra en el arreglo.");
    } else {
        println!("La cadena ingresada no se encuentra en el arreglo.");
    }
}
