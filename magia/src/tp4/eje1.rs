pub trait Primo {
    fn es_primo(&self) -> bool; //metodo abstracto
}
impl Primo for i32 {
    fn es_primo(&self) -> bool {
        // 1 siempre es divisor
        let mut cant_divisible: u32 = 1; //se puede dividir por si mismo y 1, me da un entero, solo only 2 enteros
        let mut i = 2;
        if *self > 1 {
            while i <= *self && (cant_divisible <= 2) {
                if self % i == 0 {
                    cant_divisible += 1;
                }
                i += 1;
            }
        }
        cant_divisible == 2
    }
}

fn cantidad_primos_vector(vec: Vec<i32>) -> u32 {
    let mut cant_primos: u32 = 0;
    let resuelve = |x: &i32| x.es_primo();
    for i in vec.iter() {
        if resuelve(i) {
            cant_primos += 1;
        }
    }
    return cant_primos;
}
pub fn main() {}

#[test]
fn test_es_primo() {
    assert_eq!(1.es_primo(), false);
    assert_eq!(2.es_primo(), true);
    assert_eq!(3.es_primo(), true);
    assert_eq!(4.es_primo(), false);
    assert_eq!(5.es_primo(), true);
    assert_eq!(6.es_primo(), false);
}

#[test]
fn test_cant_primos() {
    let vec = vec![1, 2, 3, 4, 5, 6]; // 2, 3, 5
    assert_eq!(cantidad_primos_vector(vec), 3);
}

#[test]
fn test_sin_primos(){
    let vec = vec![4, 6, 8, 10];
    assert_ne!(cantidad_primos_vector(vec), 1);
}