/*-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
números mayores al límite que tiene el arreglo */

fn cantidad_de_mayores(v: [i32; 5], limite: i32) -> u32 {
    let mut cantidad: u32 = 0;
    for i in 0..5 {
        if v[i] > limite {
            cantidad += 1;
        }
    }
    return cantidad;
}
pub fn main() {
    let v: [i32; 5] = [1, 2, 3, 4, 5];
    let limite = 3;
    print!("{}", cantidad_de_mayores(v, limite));
}

#[test]

fn validator() {
    let v: [i32; 5] = [1, 2, 3, 4, 5];
    let limite: i32 = 3;
    assert_eq!(cantidad_de_mayores(v, limite), 2);
}
