/* Escribir un programa que defina una variable de tipo arreglo que contenga seis números
enteros, y luego multiplique cada valor del arreglo por un valor constante definido,
modificando el contenido del arreglo. */

const CONSTANTINOPLA: i64 = 5;
fn main() {
    let mut mi_areglo: [i64; 6] = [1, 2, 3, 4, 5, 6];

    for i in 0..6 {
        mi_areglo[i] *= CONSTANTINOPLA;
    }

    for i in 0..6 {
        println!("i: {i} valor contenido: {}", mi_areglo[i]);
    }
}
