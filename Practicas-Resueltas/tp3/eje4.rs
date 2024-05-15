/*4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero,
isósceles o escaleno.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna. */

struct Triangulo {
    long_lado1: f32,
    long_lado2: f32,
    long_lado3: f32,
}
impl Triangulo {
    fn new(lado1: f32, long_lado2: f32, long_lado3: f32) -> Triangulo {
        Triangulo {
            long_lado1: lado1,
            long_lado2,
            long_lado3,
        }
    }
    /*
    1-Equilátero: Un triángulo es equilátero si todos sus lados tienen la misma longitud.
    Condición: lado1 == lado2 && lado1 == lado3
    2-Isósceles: Un triángulo es isósceles si al menos dos de sus lados tienen la misma longitud.
    Condición: lado1 == lado2 || lado1 == lado3 || lado2 == lado3
    3-Escaleno: Un triángulo es escaleno si todos sus lados tienen longitudes diferentes.
    Condición: lado1 != lado2 && lado1 != lado3 && lado2 != lado3
    */
    fn determinar_tipo(&self) -> String {
        match (
            self.long_lado1 == self.long_lado2,
            self.long_lado1 == self.long_lado3,
            self.long_lado2 == self.long_lado3,
        ) {
            (true, true, true) => "Equilatero".to_string(),
            (true, true, false) | (true, false, true) | (false, true, true) => {
                "Isosceles".to_string()
            }
            _ => "Escaleno".to_string(),
        }
    }
    //➢ calcular_area: calcular el área y la retorna.
    //Formula de Heron
    fn calcular_area(&self) -> f32 {
        let area: f32;
        let s: f32 = (self.long_lado1 + self.long_lado2 + self.long_lado3) / 2.0;

        let dentro_de_la_raiz =
            s * (s - self.long_lado1) * (s - self.long_lado2) * (s - self.long_lado3);
        // con .sqrt() aplico la raiz cuadrada
        area = dentro_de_la_raiz.sqrt();
        area
    }
    //➢ calcular_perimetro: calcula el perímetro y lo retorna.
    fn calcular_perimetro(&self) -> f32 {
        let perimetro: f32 = self.long_lado1 + self.long_lado2 + self.long_lado3;
        perimetro
    }
}
pub fn main() {}

#[cfg(test)]
mod perimetro {
    //Encapsulo los test de cada funcion
    use super::*;
    #[test]
    fn test_calcular_perimetro() {
        let triangulo = Triangulo::new(5.0, 5.0, 5.0);
        assert_eq!(triangulo.calcular_perimetro(), 15.0);
    }
    #[test]
    fn test_calcular_perimetro2() {
        let triangulo = Triangulo::new(5.0, 5.0, 5.0);
        assert_ne!(triangulo.calcular_perimetro(), 105.0);
		// assert_ne! lo uso cuando se que el resultado va ser incorrecto
    }
}

mod calcular_area_tests {
    use super::*;
    #[test]
    fn test_calcular_area() {
        let triangulo = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(triangulo.calcular_area(), 6.0);
    }
}
