pub fn main() {
    let v: [i32; 5] = [1, 2, 3, 4, 5];
    print!("{}", cantidad_impares(v));
}

fn cantidad_impares(v: [i32; 5]) -> i32 {
    let mut cantidad: i32 = 0;
    for i in 0..v.len() {
        if !v[i] % 2 == 0 {
            cantidad += 1;
        }
    }
    return cantidad;
}

#[test]

fn prueba_cantidad() {
    let v_prueba: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(cantidad_impares(v_prueba), 3);
}
