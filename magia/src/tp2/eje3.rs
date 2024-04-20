pub fn main() {
    //mutabilidad: tipo; rango = valor para cada espacio
    let arreglo: [i32; 5] = [1, 2, 3, 4, 5];

    // print!("suma de los pares {}", suma_pares(arreglo));
    // suma_pares(arreglo);
}

fn suma_pares(arreglo: [i32; 5]) -> i32 {
    let mut suma: i32 = 0;
    let mut helper: i32;
    for i in 0..arreglo.len() {
        helper = arreglo[i];
        if helper % 2 == 0 {
            suma += helper;
        }
    }
    return suma;
}

#[test]
fn pruebarda() {
    let v: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(suma_pares(v), 6);
}
