/* Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
arreglos pasados por parámetro, correspondiéndose el resultado con cada posición de los
arreglos pasados por parámetro. */
fn sumar_arreglos(v: [f32; 5], v2: [f32; 5]) -> [f32; 5] {
    let mut nuevo_arreglo: [f32; 5] = [0.0; 5];
    for i in 0..5 {
        nuevo_arreglo[i] = v[i] + v2[i];
    }
    nuevo_arreglo
}

pub fn main() {}

#[test]
fn validator() {
    let v0: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let v1: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let v3: [f32; 5] = sumar_arreglos(v0, v1);
    assert_eq!(v3, [2.0, 4.0, 6.0, 8.0, 10.0]);
}
