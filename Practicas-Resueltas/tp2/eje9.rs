/*Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
función retorna la cantidad de números del arreglo que están entre el rango de los
parámetros inferior y superior inclusive.
 */

fn cantidad_en_rango(v: [i32; 3], izq: i32, der: i32) -> i32 {
    let mut suma = 0;
    if der >= izq {
        for i in izq-1..der {
            suma += 1;
        }
    }
    suma
}
pub fn main() {}

#[test]
fn validator1() {
    let v0: [i32; 3] = [1, 2, 3];
    let cant_cumplen: i32 = 3; // Actualiza este valor con el resultado esperado
    let izq: i32 = 1;
    let der: i32 = 3;
    let cantidad = cantidad_en_rango(v0, izq, der);
    assert_eq!(
        cantidad, cant_cumplen,
        "La cantidad de números en el rango no es la esperada"
    );
}

#[test]
fn validator2() {
    let v0: [i32; 3] = [1, 2, 3];
    let izq: i32 = 1;
    let der: i32 = 3;
    let cantidad = cantidad_en_rango(v0, izq, der);
    assert_eq!(cantidad, 3);
}
