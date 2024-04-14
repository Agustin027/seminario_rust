use std::io;
fn main() {
  const num: i32 = 5;
  let mut arreglo = [10,20,30,40,50,60];
  for i in 0..6{
      arreglo[i]=  arreglo[i]*num;
  }
  
   println!("Arreglo modificado: {:?}", arreglo);
  }