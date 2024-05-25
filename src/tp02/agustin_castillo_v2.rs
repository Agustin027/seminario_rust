/* esto no tenia que estar en el entregable
fn main() {
    //Nombre:Agustin Castillo; Dni:44130476; Legajo:20778/1; Alias: Agustin;
    let cantidades = [5, 10, 15, 20];
    let precios = [1.5, 2.5, 3.5, 4.5];
    let porcentaje = 0;
    println!(
        "{}",
        calcular_precio_con_impuestos(cantidades, precios, porcentaje)
    );
    println!(
        "{}",
        calc_precio_con_impuestos_zip(cantidades, precios, porcentaje)
    );
}*/
fn calcular_precio_con_impuestos(cantidades: [u32; 4], precios: [f32; 4], porcentaje: u32) -> f32 {
    let mut total = 0.0;
    let p = porcentaje as f32 / 100.0;

    for i in 0..4 {
        total = total
            + ((cantidades[i] as f32 * precios[i]) + ((cantidades[i] as f32 * precios[i]) * p))
    }
    println!("Con For: {}", total);
    total
}

fn calc_precio_con_impuestos_zip(cantidades: [u32; 4], precios: [f32; 4], porcentaje: u32) -> f32 {
    let p = porcentaje as f32 / 100.0;

    let total = cantidades
        .iter()
        .zip(precios.iter())
        .map(|(&cantidad, &precio)| cantidad as f32 * precio + cantidad as f32 * precio * p)
        .sum();
    println!("Con Zip: {}", total);
    total
}

#[test]
fn test_val_altos() {
    let cantidades = [5000, 1000, 15000, 20000];
    let precios = [100.55, 200.99, 35.99, 40.55];
    let porcentaje = 15;

    assert_eq!(
        calcular_precio_con_impuestos(cantidades, precios, porcentaje),
        2362778.5
    );

    assert_eq!(
        calc_precio_con_impuestos_zip(cantidades, precios, porcentaje),
        2362778.5
    );
}
#[test]
fn test_p_cero() {
    let cantidades = [5, 10, 15, 20];
    let precios = [1.5, 2.5, 3.5, 4.5];
    let porcentaje = 0;

    assert_eq!(
        calcular_precio_con_impuestos(cantidades, precios, porcentaje),
        175.0
    );

    assert_eq!(
        calc_precio_con_impuestos_zip(cantidades, precios, porcentaje),
        175.0
    );
}
#[test]
fn test_cero() {
    let cantidades = [0, 0, 0, 0];
    let precios = [0.0, 0.0, 0.0, 0.0];
    let porcentaje = 0;

    assert_eq!(
        calcular_precio_con_impuestos(cantidades, precios, porcentaje),
        0.0
    );

    assert_eq!(
        calc_precio_con_impuestos_zip(cantidades, precios, porcentaje),
        0.0
    );
}
/* Feedback del profe
"Hola Agustin, la nota del entregable#1 es 7.50 te paso dar el feedback del mismo:
1- uso excesivo de println! Esto lo comente en reiteradas oportunidades, solo lo usamos a modo educativo.
2- no es necesario que haya codigo en main o que este el main eso no se pedia
3- la funcion calc_precio_con_impuestos_zip no se utiliza en la funcion pedida y no logro comprender porque esta en la v2
4- bien que hayas sumado mas casos de tests en la v2
5- en cuanto al tamano de los arrays que sean fijos hubiese sido mejor pasar la ref & para que no quede acotado al size
6- idem anterior con el for podrias utilizar el .len()
"  */
