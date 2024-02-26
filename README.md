<h1 align="center">Seminario-De-Lenguajes-Rust</h1>
<p align="center">
  <img src= "https://i.giphy.com/nyTiFWr5VtkRDOa1Cd.webp" autoplay alt="Descripción de la imagen">
</p>

<h1 align="center"> a tener en cuenta </h1>

```rs

  fn main() {
    println!("sarsa");
  }

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
  Imprimir: " println!() "

  ------Variables --------
  // Declaración de una variable mutable
  let mut variable_mutable = 42;

  // Declaración de una variable inmutable
  let variable_inmutable = 20;

  ------tipos de datos --------
  let x = 5; // Variable entera inmutable
  let y = 3.5; // Variable flotante inmutable
  let z = "Hola, mundo!"; // Variable de cadena inmutable
  const PI: f64 = 3.14159; // Constante
  //let const //diferencia cuando quiero una variable inmutable en su ambito local y constante global inmutable
```

<!-- -->
<table align="center">
  <thead>
    <tr>
      <th>Longitud</th>
      <th>Con Signo</th>
      <th>Sin Signo</th>
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
    </tr>
    <tr>
      <td>64-bit</td>
      <td><code>i64</code></td>
      <td><code>u64</code></td>
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
  
  loop {   //Bucle Loop (Bucle infinito):
      println!("Contador: {}", contador);
	  contador += 1;

	  if contador >= 5 {
	     break; // Termina el bucle cuando el contador es mayor o igual a 5
	  }
  }
``` 
