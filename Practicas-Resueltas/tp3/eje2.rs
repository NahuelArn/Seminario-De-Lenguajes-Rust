/* Escribir un programa que defina la estructura Rectángulo que tenga campos para la
longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo
retorna.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.
➢ es_cuadrado: retorna true si es cuadrado, false caso contrario
 */

struct Rectangulo {
    longitud: f32,
    ancho: f32,
}
impl Rectangulo {
    fn new(longitud: f32, ancho: f32) -> Rectangulo {
        Rectangulo { longitud, ancho }
    }

    fn calcular_area(&self) -> f32 {
        return self.longitud * self.ancho;
    }

    fn calcular_perimetro(&self) -> f32 {
        return (self.longitud * 2.0) + (self.ancho * 2.0);
    }

    fn es_cuadrado(&self) -> bool {
        return self.longitud == self.ancho;
    }
}
pub fn main() {
    let rectangulo = Rectangulo::new(5.0, 5.0);
    println!("Area del rectángulo: {}", rectangulo.calcular_area());
    println!(
        "Perimetro del rectángulo: {}",
        rectangulo.calcular_perimetro()
    );
    println!("Es cuadrado: {}", rectangulo.es_cuadrado());
}

#[test]
fn test_new_rectangulo() {
    let rectangulo = Rectangulo::new(5.0, 10.0);
    assert_eq!(rectangulo.longitud, 5.0);
    assert_eq!(rectangulo.ancho, 10.0);
}

#[test]
fn test_calcular_area() {
    let rectangulo = Rectangulo::new(3.0, 4.0);
    assert_eq!(rectangulo.calcular_area(), 12.0);
}

#[test]
fn test_calcular_perimetro() {
    let rectangulo = Rectangulo::new(3.0, 4.0);
    assert_eq!(rectangulo.calcular_perimetro(), 14.0);
}

#[test]
fn test_es_cuadrado() {
    let rectangulo1 = Rectangulo::new(4.0, 4.0);
    assert_eq!(rectangulo1.es_cuadrado(), true);

    let rectangulo2 = Rectangulo::new(3.0, 4.0);
    assert_eq!(rectangulo2.es_cuadrado(), false);
}
