/*Definir una función llamada incrementar que recibe como parámetro un número flotante
e incrementa en 1 su valor. */

fn incrementar(num: &mut f32) {
    *num += 1.0; //el * es para desreferenciar
}
pub fn main() {
    let mut num: f32 = 1.0;
    incrementar(&mut num);
    print!("{}", num);
}

#[test]

fn validator_true() {
    let mut num: f32 = 1.0;
    incrementar(&mut num);
    assert_eq!(num, 2.0);
}

#[test]

fn validator_false() {
    let mut num: f32 = 1.0;
    incrementar(&mut num);
    assert_eq!(num, 3.0);
}
