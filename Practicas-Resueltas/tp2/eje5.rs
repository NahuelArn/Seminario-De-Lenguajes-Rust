/*-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro. */
fn duplicar_valores(v: [f32; 5]) -> [f32; 5] {
    let mut nuevo_arreglo: [f32; 5] = [0.0; 5];

    for i in 0..5 {
        nuevo_arreglo[i] = v[i] * 2.0;
    }
    nuevo_arreglo
}

//los _ lo uso para ignorar las advertencias de q no estoy usando las variables
//tambien se puede implementar con esto #[allow(unused_variables)]antes del main
pub fn main() {
    let v: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let _nuevo_arreglo: [f32; 5];

    _nuevo_arreglo = duplicar_valores(v);

    for i in 0..5 {
        print!("{} ", _nuevo_arreglo[i]);
    }
}

// }
#[test]
fn validator() {
    let v: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let resultado: [f32; 5] = duplicar_valores(v);

    assert_eq!(resultado, [2.0, 4.0, 6.0, 8.0, 10.0]);
}
