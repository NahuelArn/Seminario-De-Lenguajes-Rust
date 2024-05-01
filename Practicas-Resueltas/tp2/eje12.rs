/*Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
reemplaza todos los números pares por -1 */

fn reemplazar_pares(v: &mut [i32; 5]) {
    fn es_par(num: i32) -> bool {
        return num % 2 == 0;
    }
    for i in 0..5 {
        if es_par(v[i]) {
            v[i] = -1;
        }
    }
}

pub fn main() {}

#[test]
fn validator_true() {
    let mut v: [i32; 5] = [1, 2, 3, 4, 5];
    reemplazar_pares(&mut v);
    assert_eq!(v, [1, -1, 3, -1, 5]);
}

#[test]
fn validator_false() {
    let mut v: [i32; 5] = [1, 2, 3, 4, 5];
    reemplazar_pares(&mut v);
    assert_eq!(v, [1, 2, 3, -1, 5]);
}
