use std::io;
fn main() {
  let mut numeroU: u32 =5;
  
 println!("Ingrese un numero");
 let mut input = String::new();
 io::stdin().read_line(&mut input).expect("error");
 let mut num2: i32 = input.trim().parse().expect("error");
 
 
 let mut result = numeroU as i32+num2; 
 result=result*result;
 
 println!("El valor del numero elevado al cuadrado es: {}",result)
}