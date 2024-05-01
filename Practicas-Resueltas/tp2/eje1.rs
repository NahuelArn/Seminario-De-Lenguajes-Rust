fn es_par(num: i32) -> bool {
    let help: bool;
    if num % 2 == 0 {
        help = true;
    } else {
        help = false;
    }
    help
}

pub fn main() {
    println!("{}", es_par(2));
}

#[test]
fn validador() {
    for i in 0..6 {
        assert_eq!(es_par(i), true);
    }
}

#[test]
fn validador2() {
    assert_eq!(es_par(1), true);
}

#[test]
fn validador_numeros_negativos() {
    for i in -5..0 {
        assert_eq!(es_par(i), i % 2 == 0);
    }
}

#[test]
fn validador_numeros_grandes() {
    for i in 1_000_000..1_000_005 {
        assert_eq!(es_par(i), i % 2 == 0);
    }
}

#[test]
fn validador_cero() {
    assert_eq!(es_par(0), true);
}

#[test]
fn validador_multiples_numeros() {
    let numeros = vec![2, 4, 6, 8, 10];
    for &num in &numeros {
        assert_eq!(es_par(num), true);
    }
}

#[test]
fn validador_impares() {
    for i in 1..6 {
        assert_eq!(es_par(i * 2 - 1), false);
    }
}
