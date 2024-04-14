use std::io;
fn main() {
    let ok: bool=true;
    println!("Ingresar un boolean");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let ok2 : bool= input.trim().parse().expect("Error");
    
   println!("Resultado de ok AND ok2 {}", ok && ok2);
    println!("Resultado de ok OR ok2 {}", ok || ok2);
    
}