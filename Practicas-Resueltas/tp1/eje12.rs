/*- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de
enteros, y luego imprima la cadena y la suma de los valores en el arreglo.  */
fn sumarda(v: [i32; 5]) -> i32 {
    let mut suma: i32 = 0;
    for i in 0..5 {
        suma += v[i];
    }
    suma
}
fn main() {
    let v: [i32; 5] = [1, 2, 3, 4, 5];
    let tupla: (String, [i32; 5]) = (String::from("casa"), v);

    println!(
        "cadena: {} suma vector: {}",
        tupla.0,
        sumarda(v).to_string()
    );
}
