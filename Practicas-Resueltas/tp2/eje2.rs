fn es_primo(num: i32) -> bool {
    let mut condicion = false; //se puede dividir por si mismo y 1, me da un entero, solo only 2 enteros
    let mut cant_divisble: u32 = 0;
    let mut i = 1;
    if num > 1 {
        while i < num && !condicion && (cant_divisble < 2) {
            if !(num % i == 0) {
                condicion = true;
            } else {
                cant_divisble += 1;
                i += 1;
            }
        }
    }
    return condicion;
}

pub fn main() {
    es_primo(1);
}

#[test]
fn valor_primo() {
    assert_eq!(es_primo(5), true);
}

#[test]
fn valor_invalido() {
    assert_eq!(es_primo(1), true);
}
