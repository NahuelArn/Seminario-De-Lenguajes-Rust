/*Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
del arreglo que son de longitud mayor al parámetro límite. */

fn cantidad_de_cadenas_mayor_a(v: &[String], limite: usize) -> u32 {
    //te pide un arreglo de Strings, No un arreglo a referencias de Strings
    let mut cantidad: u32 = 0;
    for i in v {
        if i.len() > limite {
            cantidad += 1;
        }
    }
    cantidad
}
pub fn main() {
    let v: [String; 3] = [
        String::from("casa1"),
        String::from("casa22"),
        String::from("casa3"),
    ];

    print!("{}", cantidad_de_cadenas_mayor_a(&v, 5));
}

#[test]

fn validator() {
    let v: [String; 3] = [
        String::from("casa1"),
        String::from("casa22"),
        String::from("casa3"),
    ];
    assert_eq!(cantidad_de_cadenas_mayor_a(&v, 5), 1);
}
