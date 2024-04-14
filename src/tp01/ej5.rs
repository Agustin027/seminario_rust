use std::io;
fn main() {
   let mut cadena:String="Messi".to_string();


   println!("Ingresar un string");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Error al leer la entrada.");

    
    
    cadena.push_str(input.trim());

    // Imprimir la cadena en mayúsculas
    println!("La cadena concatenada en mayúsculas es: {}", cadena.to_uppercase());
 
}