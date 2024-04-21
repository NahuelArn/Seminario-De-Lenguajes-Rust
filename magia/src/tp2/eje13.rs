/*Definir una función llamada ordenar_nombres que recibe un arreglo de String y los
ordena en orden alfabético.
 */

fn ordenar_nombres(v: &mut [String]) {
    v.sort();
}

pub fn main() {
    let mut v: [String; 5] = [
        String::from("casa5"),
        String::from("casa4"),
        String::from("casa7"),
        String::from("casa2"),
        String::from("casa1"),
    ];
    ordenar_nombres(&mut v);

    for i in v {
        print!("{}", i);
    }
}

#[test]

fn validator() {
    let mut v: [String; 5] = [
        String::from("casa5"),
        String::from("casa4"),
        String::from("casa7"),
        String::from("casa2"),
        String::from("casa1"),
    ];
    ordenar_nombres(&mut v);

    let v_ordenado = [
        String::from("casa1"),
        String::from("casa2"),
        String::from("casa4"),
        String::from("casa5"),
        String::from("casa7"),
    ];

    assert_eq!(v, v_ordenado);
}
