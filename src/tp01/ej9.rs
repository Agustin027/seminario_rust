use std::io;

fn main() {
    let arreglo = [1,2,3,4,5];
    let mut total: i32=0;
    for i in 0..5{
      total+=  arreglo[i];
  }
  
  println!("Resultado: {}",total);
}
