/*Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
por el parámetro factor modificándolo.
 */

// fn multiplicar_valores(v: &mut [i32; 5], factor: u32) {  Sigue la misma logica de Java, con un for each no podes usarlo para modificar valores
//     for i in v {
//         println!("{}", i);
//     }
// }

fn multiplicar_valores(v: &mut [i32; 5], factor: i32) {
    for i in 0..5 {
        v[i] *= factor;
    }
}

pub fn main() {
    let mut v: [i32; 5] = [1, 2, 3, 4, 5];
    multiplicar_valores(&mut v, 2);

    for i in v {
        println!("{}", i);
    }
}

#[test]

fn validator() {
    let mut v: [i32; 5] = [1, 2, 3, 4, 5];
    multiplicar_valores(&mut v, 2);

    assert_eq!(v, [2, 4, 6, 8, 10]);
}
