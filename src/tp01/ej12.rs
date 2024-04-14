fn main() {
    let tupla=("Hola", [1, 2, 3, 4, 5]);
    println!("Cadena: {}", tupla.0);
    
    let arreglo=tupla.1;
    
    let mut total: i32=0;
    for i in 0..5{
      total+=  arreglo[i];
     }
  println!("Resultado: {}",total);

}
