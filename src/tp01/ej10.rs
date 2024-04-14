fn main() {
    let arreglo1 = [1,2,3,4,5];
    let arreglo2 = [6,7,8,9,10];
    let mut suma_arreglos = [0; 5];
  

  for i in 0..5 {
        suma_arreglos[i] = arreglo1[i] + arreglo2[i];
    }
      println!("El tercer arreglo es: {:?}", suma_arreglos);
}
