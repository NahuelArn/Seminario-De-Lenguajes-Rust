use std::io;

fn main() {
    let var_a = 5;

    println!("Ingrese un numero");

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .ok()
        .expect("Fallo de linea");

    println!("sarasa {}", entrada);

    //convertimos la entrada
    let numero: i32 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error, ingrese un numero valido i32");
            return;
        }
    };
    println!("pepe {}", numero + 2);
}
