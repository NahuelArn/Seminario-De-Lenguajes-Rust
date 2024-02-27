/* Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la
suma de los valores del  arreglo.
*/

fn main() {
    let arreglo: [i64; 5] = [1, 2, 3, 4, 5];  //primero especifico el tipo y la dimF del array, despues cargo el contendio manualmente

    let mut sumador: i64 = 0;

    for i in 0..5 {
        sumador += arreglo[i];
    }
    println!("La suma de los elementos del arreglo es: {}", sumador);
}
