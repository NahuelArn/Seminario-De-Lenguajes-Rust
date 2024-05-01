/*Crear una función en Rust llamada 'calcular_precio_con_impuestos' que calcule el total de una compra de productos,
incluyendo un impuesto global.

La función debe recibir los siguientes parámetros:

Un arreglo llamado 'cantidades' de las cantidades de productos comprados.
Un arreglo llamado 'precios' de los precios unitarios de los productos comprados.
Un valor de porcentaje de impuesto a aplicar sobre la compra.

Los valores de las posiciones de los parametros 'cantidades' y 'precios' se corresponden

La función debe retornar el total de la compra con impuestos aplicados.

Además, se solicita que la función contenga los tests necesarios para asegurar su correcto funcionamiento. */

// const IMPUESTO_GLOBAL: f64 = 3.14159; // Constante

fn calcular_precio_con_impuestos(cantidades: [u32; 5], precios: [f32; 5], porcentaje: f32) -> f32 {
    let mut total: f32 = 0.0;
    let mut aux: f32;
    for i in 0..5 {
        aux = cantidades[i] as f32;
        total += aux * precios[i];
    }

    let impuesto = total * porcentaje as f32 / 100.0;
    total += impuesto;

    total
}

pub fn main() {
    let cantidades: [u32; 5] = [1, 2, 3, 4, 5];
    let precios: [f32; 5] = [100.0, 200.0, 300.0, 400.0, 500.0];
    let porcentaje: f32 = 15.5; //me aplican un 15.5 % sobre mi compra

    // let totol_a_pagar: f32; NO ME PIDE ESTO, podria llamar solo a la funcion y no guardarme su valor
    let totol_a_pagar: f32 = calcular_precio_con_impuestos(cantidades, precios, porcentaje);

    println!("{}", totol_a_pagar);
}

#[test]

fn validator_true() {
    let cantidades: [u32; 5] = [1, 2, 3, 4, 5];
    let precios: [f32; 5] = [100.0, 200.0, 300.0, 400.0, 500.0];
    let porcentaje: f32 = 15.5; //me aplican un 15.5 % sobre mi compra

    // let totol_a_pagar: f32; NO ME PIDE ESTO, podria llamar solo a la funcion y no guardarme su valor
    assert_eq!(
        calcular_precio_con_impuestos(cantidades, precios, porcentaje),
        6352.5
    );
}

#[test]

fn validator_false() {
    let cantidades: [u32; 5] = [1, 2, 3, 4, 5];
    let precios: [f32; 5] = [100.0, 200.0, 300.0, 400.0, 500.0];
    let porcentaje: f32 = 15.5; //me aplican un 15.5 % sobre mi compra

    // let totol_a_pagar: f32; NO ME PIDE ESTO, podria llamar solo a la funcion y no guardarme su valor
    assert_eq!(
        calcular_precio_con_impuestos(cantidades, precios, porcentaje),
        6352.0
    );
}

/*Paso todos los test q se me ocurrieron en el momento
  lo probe con un error y lo detecto, tambien para el caso que estaria bien
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/