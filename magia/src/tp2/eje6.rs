fn longitud_de_cadenas(v: [&str; 5]) -> [usize; 5] {
    let mut vector_retorno: [usize; 5] = [0; 5];
    let mut cadena_actual: String;

    for i in 0..5 {
        cadena_actual = String::from(v[i]);
        vector_retorno[i] = cadena_actual.len();
    }
    vector_retorno
}
pub fn main() {
    let vector: [&str; 5] = ["caza"; 5];

    let vector_nuevo: [usize; 5] = longitud_de_cadenas(vector);

    // for i in 0..5 {
    //     println!("{} ", vector_nuevo[i]);
    // }
}
