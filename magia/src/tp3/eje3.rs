/*3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el
mes y el año. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
➢ es_fecha_valida: retorna true si es una fecha válida, false caso contrario.//tenga en
cuenta los años bisiestos también.
➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
la fecha pasada por parámetro. */

struct Fecha {
    dia: u32,
    mes: u32,
    anho: u32,
}

impl Fecha {
    //➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
    fn new(dia: u32, mes: u32, anho: u32) -> Fecha {
        Fecha { dia, mes, anho }
    }
    //➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
    fn es_bisiesto(&self) -> bool {
        let estado: bool = (self.anho % 4 == 0) && (self.anho % 100 != 0) || (self.anho % 400 == 0);
        return estado;
    }
    //helper que le paso un mes y me tira la cantidad de dias que tiene ese mes, incluso evalua si el anho es bisiesto
    fn cantidad_de_dias(&self, mes: u32) -> u32 {
        //funcion helper
        let dias_del_mes: u32 = match mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.es_bisiesto() {
                    29
                } else {
                    28
                }
            }
            _ => return 0,
        };
        dias_del_mes
    }
    //➢ es_fecha_valida: retorna true si es una fecha válida, false caso contrario.//tenga en cuenta los años bisiestos también.
    fn _es_fecha_valida(&self) -> bool {
        let mut estado: bool = false;
        if self.mes > 0 && self.mes < 13 && self.anho > 0 {
            println!("mes::: {}", self.mes);
            let dias_del_mes: u32 = self.cantidad_de_dias(self.mes);
            println!("cumpleeasdasd::: {}", dias_del_mes);
            println!("fechaEséradpda: {}", self.dia);
            estado = dias_del_mes != 0 && (self.dia >= 1 && self.dia <= dias_del_mes);
        }
        println!("{}", estado);
        estado
    }
    //➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
    fn _sumar_dias(&mut self, dias: u32) {
        let mut dias_del_mes = self.cantidad_de_dias(self.mes);
        self.dia += dias;

        // Mientras el dia actual sea mayor que la cantidad de dias en el mes actual
        while self.dia > dias_del_mes {
            // Restamos la cantidad de dias en el mes actual
            self.dia -= dias_del_mes;
            // Avanzamos al siguiente mes
            self.mes += 1;

            // Si el mes actual supera diciembre, avanzamos al siguiente año
            if self.mes > 12 {
                self.mes = 1;
                self.anho += 1;
            }
            // Actualizamos la cantidad de días en el nuevo mes
            dias_del_mes = self.cantidad_de_dias(self.mes);
        }
    }
    //➢ restar_dias(dias): resta la cantidad de dias a la fecha, modificandose
    fn restar_dias(&mut self, mut dias: u32) {
        // Mientras haya días para restar
        while dias != 0 {
            // Si la cantidad de dias a restar es mayor o igual que el dia actual
            if dias >= self.dia {
                // Restamos el día actual
                dias -= self.dia;
                // Retrocedemos al mes anterior
                self.mes -= 1;
                // Si el mes es menor que 1, retrocedemos al año anterior
                if self.mes < 1 {
                    self.mes = 12;
                    self.anho -= 1;
                }
                // Actualizamos la cantidad de dias en el nuevo mes
                self.dia = self.cantidad_de_dias(self.mes);
            } else {
                // Si la cantidad de dias a restar es menor que el dia actual,
                // simplemente restamos esa cantidad de días
                self.dia -= dias;
                dias = 0; // Ya no quedan dias por restar
            }
        }
    }
    //➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a la fecha pasada por parámetro.
    fn _es_mayor(&self, otra_fecha: &Fecha) -> bool {
        let mut mayor: bool = false;
        if self.anho > otra_fecha.anho {
            mayor = true;
        } else if self.anho == otra_fecha.anho && self.mes > otra_fecha.mes {
            mayor = true;
        } else if self.anho == otra_fecha.anho
            && self.mes == otra_fecha.mes
            && self.dia > otra_fecha.dia
        {
            mayor = true;
        }
        mayor
    }

    //esta funcion asociada no me la piden.. pero para probar
    fn to_string(&self) -> String {
        return format!("dia: {} mes: {} anho: {}", self.dia, self.mes, self.anho);
    }
}

pub fn main() {
    let mut _cumpleanhos = Fecha::new(5, 1, 2000);
    println!("{}", _cumpleanhos.to_string());
    _cumpleanhos.restar_dias(365);
    println!("{}", _cumpleanhos.to_string());

    println!("fecha valida? {}", _cumpleanhos._es_fecha_valida())
}

#[test]
fn test_es_bisiesto() {
    //deberian ser 2 test separados,igual andan, q solo me da la data de uno (si falla me dice igual)
    // Año bisiesto
    assert_eq!(Fecha::new(1, 1, 2020).es_bisiesto(), true);
    // Año no bisiesto
    assert_eq!(Fecha::new(1, 1, 2021).es_bisiesto(), false);
}

#[test]
fn test_es_fecha_valida() {
    //pasan los 2
    // Fecha válida
    assert_eq!(Fecha::new(1, 1, 2022)._es_fecha_valida(), true);
    // Fecha inválida
    assert_eq!(Fecha::new(32, 2, 2022)._es_fecha_valida(), false);
}

#[test]
fn test_sumar_dias() {
    let mut fecha = Fecha::new(31, 12, 2022);
    fecha._sumar_dias(1);
    assert_eq!(fecha.to_string(), "dia: 1 mes: 1 anho: 2023");
    // Suma de días que pasan a otro mes
    let mut fecha = Fecha::new(30, 12, 2022);
    fecha._sumar_dias(3);
    assert_eq!(fecha.to_string(), "dia: 2 mes: 1 anho: 2023");
}

#[test]
fn test_restar_dias() {
    let mut fecha = Fecha::new(1, 1, 2023);
    fecha.restar_dias(1);
    assert_eq!(fecha.to_string(), "dia: 31 mes: 12 anho: 2022");
    // Resta de días que pasan a otro mes
    let mut fecha = Fecha::new(1, 1, 2023);
    fecha.restar_dias(3);
    assert_eq!(fecha.to_string(), "dia: 29 mes: 12 anho: 2022");
}

#[test]
fn test_es_mayor() {
    let fecha1 = Fecha::new(1, 1, 2023);
    let fecha2 = Fecha::new(31, 12, 2022);
    assert_eq!(fecha1._es_mayor(&fecha2), true);
}

#[test]
fn test_es_menor() {
    let fecha1 = Fecha::new(1, 1, 2023);
    let fecha2 = Fecha::new(31, 12, 2022);
    assert_eq!(fecha2._es_mayor(&fecha1), false);
}

/* otra enunciado
struct Fecha {
    dia: u32,
    mes: u32,
    anho: u32,
}

impl Fecha {
    //➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
    fn new(dia: u32, mes: u32, anho: u32) -> Fecha {
        Fecha { dia, mes, anho }
    }
    //➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
    fn es_bisiesto(&self) -> bool {
        let estado: bool = (self.anho % 4 == 0) && (self.anho % 100 != 0) || (self.anho % 400 == 0);
        return estado;
    }
    //➢ es_fecha_valida: retorna true si es una fecha válida, false caso contrario.//tenga en cuenta los años bisiestos también.
    fn es_fecha_valida(&self) -> bool {
        let mut estado: bool = false;
        if self.dia > 0 && self.dia < 32 {
            if self.mes > 0 && self.mes < 13 {
                if self.anho > 0 {
                    estado = true;
                }
            }
        }
        return estado;
    }
    //➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
    fn sumar_dias(&mut self, dias: u32) {
        self.dia += dias;
    }
    //➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
    fn restar_dias(&mut self, dias: u32) {
        if (self.dia - dias < 0) {
            self.dia = 0;
        } else {
            if (self.dia > 0) {
                self.dia -= dias;
            }
        }
    }
    //➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a la fecha pasada por parámetro.
    fn es_mayor(&self, una_fecha: u32) -> bool {
        return self.anho > una_fecha;
    }
}
 */
