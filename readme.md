<h1 align="center">Seminario-De-Lenguajes-Rust</h1>
<p align="center">
  <img src= "https://i.giphy.com/nyTiFWr5VtkRDOa1Cd.webp" autoplay alt="Descripción de la imagen">
</p>

<h1 align="center"> a tener en cuenta </h1>

```rs

  fn main() {
    println!("sarsa");
  }
  //Convencion snake case sarasa_sarasa_pepe
  comentario 1 linea: " // "
  comentario > 1: " /* */ "
  Asignacion:  " = "
  Igualdad: " == "
  Desigualdad: " != "
  Mayor: " > "
  Menor: " < "
  Mayor o igual: " >= "
  Menor o igual: " <= "
  Y: " && "
  O: " || "
  Negación: " ! "
  Verdadero: " true "
  Falso: " false "
  booleano: " bool "
  caracter: " char "
  Imprimir: " println!() "

  ------Variables --------
  // Declaración de una variable mutable
  let mut variable_mutable = 42;

  // Declaración de una variable inmutable
  let variable_inmutable = 20;

  ------tipos de datos --------
  -str: Es un tipo de cadena de caracteres inmutable y de longitud fija.
  -String: Es un tipo de cadena de caracteres que es mutable y de longitud variable.

  let x:u32 = 5; // Variable entera inmutable
  let y:f32 = 3.5; // Variable flotante inmutable
  let z:String = "saasasra"; // Variable de cadena inmutable
  const PI: f64 = 3.14159; // Constante
  //let const //diferencia cuando quiero una variable inmutable en su ambito local y constante global inmutable
```

<h1 align="center"> parseos </h1>

```rs
  //parseos
  fn main(){
      let a: u8 = 0;
    
      let b: i32 = a as i32; // casting entre números
      let b_u8: u8 = b as u8; // casting de i32 a u8
      println!("b: {}", b);   // b: 0
      println!("b_u8: {}", b_u8); // b_u8: 0
        
      // Convertir un número a una cadena
      let numero = 5;
      let numero_str = numero.to_string(); // convierte un número a una cadena
        
      println!("Número como cadena: {}", numero_str); // Numero como cadena: 5
  }

```


<h1 align="center"> comportamiento de los Strings </h1>

```rs

  fn main() {
      
      -String: Es un tipo de cadena de caracteres que es mutable y de longitud variable.

      let mut a: String = String::from("pepe");   //mutable
      println!("{}", a);
      let casa: &str = " soy mutable";    //inmutable
      a+= casa;   //le asigno a(String mutable), una referencia de una cadena inmutable
      println!("{}", a);
      println!("-------------------");

      -str: Es un tipo de cadena de caracteres inmutable y de longitud fija.

      let mut b: &str = "moni";   //referencia inmutable
      println!("{}", b);
      b+= " soy mutable"; //error! b es inmutable es una cadena fija
      println!("{}", b)  //error!
  }

```

<h1 align="center"> pedir por teclado </h1>

```rs
  //siempre lo que se pide se guarda como String
  use std::io::stdin;
  fn main() {
      println!("Ingrese su nombre: ");
      let mut nombre = String::new();
      stdin().read_line(&mut nombre).expect("Error al leer el nombre.");
      println!("Hola, {}!", nombre);
  }


  //parsear entrada String -> i32
  use std::io;

  fn main() {
      println!("ingrese un número entero:");

      let mut input = String::new();
      io::stdin().read_line(&mut input).expect("Fallo al leer la línea");

      let numero: i32 = input.trim().parse()  //trim elimina los espacios en blanco al principio y al final de la cadena
          .expect("ingrese un número válido");

      println!("Número ingresado: {}", numero);
  }


  //Parseo de un número de punto flotante (f64) parsear entrada String -> f64
  use std::io;

  fn main() {
      println!("ingrese un número de punto flotante:");

      let mut input = String::new();
      io::stdin().read_line(&mut input)
          .expect("Fallo al leer la línea");

      let numero: f64 = input.trim().parse()
          .expect("ingrese un número válido");

      println!("Número ingresado: {}", numero);
  }


  //Parseo de un carácter (char)
  
  use std::io;

  fn main() {
      println!("Por favor ingrese un carácter:");

      let mut input = String::new();
      io::stdin().read_line(&mut input)
          .expect("Fallo al leer la línea");

      let caracter: char = input.trim().chars().next()
          .expect("Por favor ingrese un carácter válido");

      println!("Carácter ingresado: {}", caracter);
  }

```

<!-- -->
<table align="center">
  <thead>
    <tr>
      <th>Longitud</th>
      <th>Con Signo</th>
      <th>Sin Signo</th>
      <th>Flotante +- </th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>8-bit</td>
      <td><code>i8</code></td>
      <td><code>u8</code></td>
    </tr>
    <tr>
      <td>16-bit</td>
      <td><code>i16</code></td>
      <td><code>u16</code></td>
    </tr>
    <tr>
      <td>32-bit</td>
      <td><code>i32</code></td>
      <td><code>u32</code></td>
      <td><code>f32</code></td>
    </tr>
    <tr>
      <td>64-bit</td>
      <td><code>i64</code></td>
      <td><code>u64</code></td>
      <td><code>f64</code></td>
    </tr>
    <tr>
      <td>128-bit</td>
      <td><code>i128</code></td>
      <td><code>u128</code></td>
    </tr>
    <tr>
      <td>Arquitectura Cpu 32-64</td>
      <td><code>isize</code></td>
      <td><code>usize</code></td>
    </tr>
  </tbody>
</table>

<!-- -->
<h1 align="center"> Estructuras de control </h1>

```rs

  while contador < 5 {
      Bloque de codigo
  }

  for numero in 0..5 {
      Bloque de codigo
  }

  if numero < 0 {
  	  println!("El número es negativo");
  } else if{
      bloque de codigo
  }

  loop {   //Bucle Loop (Bucle infinito):   //tiene un version con tags, cuando anidas varios loops cual queres que termine "'lopardo: loop" y "break 'lopardo"
      println!("Contador: {}", contador);
	  contador += 1;

	  if contador >= 5 {
	     break; // Termina el bucle cuando el contador es mayor o igual a 5
	  }
  }

  //if con declaracion let
  let num:i32 = 5;
  let condicion_booleana: bool = num > 0;
  let data:i32 = if condicion_booleana{20}else{0};
  println!("{}", data); //20

  //MATCH
  let number = 10;
  match number {
      3 => println!("es tres o hace algo porque es 3"),
      7 => println!("es siete o hace algo porque es 7"),
      other => println!("hace algo porque porque no es 3 ni 7"), //si no es el buscado hace esto
  }
    //con place holder
    let number = 10;
    match number {
    3 => println!("es tres o hace algo porque es 3"),
    7 => println!("es siete o hace algo porque es 7"),
    _ => println!("hace algo porque porque no es 3 ni 7"), //si no es el buscado hace esto
    //_ => (), //si no es el buscado no hace nada
    }
```

<h1 align="center"> Estructuras de datos </h1>

```rs

  fn main(){
      //tupla
      let mut tupla : (String,f32,u8) = (String::from("casa"),1.0,6);
      tupla.0 = "cambio valor" .to_string();
      println!("{}", tupla.0);
      println!("{:?}", tupla);  // {:?}permite imprimir la tupla entera de forma debug (para estructuras datos compuestas)
      let(hola, flotante, entero) = tupla; // DESTRUCTURACION DE LA TUPLA ,ahora "hola" tiene "cambio valor" y cada variable tiene el valor de cada elemento de la tupla
      println!("{:?}", tupla);  //tiraria error porque ya no existe tupla


      //array
      let mut vector: [i32;5] = [1,2,3,4,5];  //[tipo,cantidad] = [elementos]
      vector[0] = 6;
      println!("{:?}", vector); //[6, 2, 3, 4, 5]

  }
  
```

<h1 align="center"> Ownership y Borrowing </h1>

```rs
  //Ownership: propiedad de un recurso, solo puede tener un dueño a la vez, "cada variable es dueña de su valor"
  /* 
  "aparece el concepto con los tipos de datos compuestos, como las cadenas de texto y los vectores etc etc."
  1. Cada valor en Rust tiene un dueño.
  2. Solo puede haber un dueño a la vez.
  3. Cuando el dueño queda fuera del alcance, el valor se eliminará.*/

  fn main(){
      let s1 = 10;
      let s2 = s1; //s1 se copia en s2, porque es un tipo primitivo
      println!("{}", s1); // no pasa naty por ser primitivos
  }
  //para que aparesca el concepto de ownership, los tipos no tienen que implementar el trait Copy

  fn main(){
      let s1 = String::from("hola");
      let s2 = s1; //s1 se mueve a s2, s1 ya no es valido.. s2 le roba el valor a s1 y ahora s2 es el dueño... ahora s1 = basura
      println!("{}", s1); //error! s1 ya no es valido
  }
  //aca aparece el conceto de borrowing
  fn main(){
      let s1 = String::from("hola");
      let s2 = &s1; //s2 es una referencia a s1, s2 no es dueño de s1, s2 es un prestamo de s1
      println!("{}", s1); //no pasa nada.. duracion del prestamo: alcance de s1 solo en el main
  }

  Tipos implementan el trait Copy:
  ● Todos los enteros
  ● Booleanos
  ● Punto flotante
  ● Char
  ● Tupla que solo tengan los tipos que implementan Copy

```

<h1 align="center"> Tiempo de vida </h1>

```rs

  fn main() {
      let string1 = String::from("Seminario de:");
      let string2 = "Rust!!!";
      let result = mas_largo(string1.as_str(), string2);
      println!("El mas largo es:{}", result);
  }
  fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str { //explicito lifetime generico 'a para x e y , para el retorno tambien 'a 
      if x.len() > y.len() {
         x
      } else {
         y
      }
  }
  &i32 // una referencia
  &'a i32 // una referencia con explicito lifetime
  &'a mut i32 // una referencia mutable con explicito lifetime

```
