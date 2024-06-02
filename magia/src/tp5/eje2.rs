/*
 En base al ejercicio 8 del tp#3 implemente lo siguiente:
  a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
  de por lo menos 90%

  b- Una vez obtenido dicho coverage, las canciones de la playlist deben ser
  guardadas en un archivo en formato JSON, por lo tanto las operaciones que agreguen,
  quiten o modifiquen la playlist deben estar respaldadas sobre dicho archivo.

  No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
  haga métodos nuevos. Recuerde también que se debe seguir manteniendo un coverage de
  al menos 90%,
*/
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
// use std::{collections::VecDeque, io::Write};

impl PartialEq for Genero {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Genero::Rock, Genero::Rock)
            | (Genero::Pop, Genero::Pop)
            | (Genero::Rap, Genero::Rap)
            | (Genero::Jazz, Genero::Jazz)
            | (Genero::Otros, Genero::Otros) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

impl PartialEq for Cancion {
    fn eq(&self, other: &Self) -> bool {
        self.titulo == other.titulo && self.artista == other.artista && self.genero == other.genero
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}
impl Cancion {
    fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion {
            titulo,
            artista,
            genero,
        }
    }
}

struct Playlist {
    nombre_de_playlist: String,
    lista: Vec<Cancion>,
}
impl Playlist {
    fn new(nombre_de_playlist: String, lista: Vec<Cancion>) -> Playlist {
        Playlist {
            nombre_de_playlist,
            lista,
        }
    }
    //➔ agregar canción.
    fn agregar_cancion(&mut self, cancion: Cancion) {
        self.lista.push(cancion);
        self.guardar_en_archivo();
    }
    //➔ eliminar canción.
    fn eliminar_cancion(&mut self) {
        self.lista.pop();
        self.guardar_en_archivo();
    }
    //➔ mover canción // mueve la canción a una determinada posición de la playlist.
    fn mover_cancion_pos(&mut self, cancion: Cancion, posicion: usize) {
        let mut indice: usize = posicion;
        if posicion > 0 {
            indice = posicion - 1;
        };
        if self.lista.len() > 0 && indice < self.lista.len() {
            self.lista.insert(indice, cancion); //insert: inserta en la posicion y mueve todo para la derecha
        }
        self.guardar_en_archivo();
    }
    //     ➔ buscar canción por nombre.
    //no especifica podria ser devolver la cancion o devolver un boolean si encontro o no
    fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<&Cancion> {
        let dim_f: usize = self.lista.len();
        let mut i: usize = 0;
        let mut encontre: bool = false;
        if dim_f > 0 {
            while i < dim_f && !encontre {
                if self.lista[i].titulo == nombre {
                    encontre = true;
                }
                i += 1;
            }
        }
        if encontre {
            i -= 1;
            return Some(&self.lista[i]);
        } else {
            None
        }
    }
    //     ➔ obtener las canciones de un determinado género.
    fn obter_canciones_de_un_determinado_genero(&self, genero: Genero) -> Vec<&Cancion> {
        let mut canciones_del_genero = Vec::new();
        if self.lista.len() > 0 {
            for i in &self.lista {
                if i.genero == genero {
                    canciones_del_genero.push(i);
                }
            }
        }
        canciones_del_genero
    }
    //     ➔ obtener las canciones de un determinado artista.
    fn obtener_canciones_de_un_determinado_artista(&self, artista: &str) -> Vec<&Cancion> {
        let mut canciones_del_artista = Vec::new();
        for cancion in &self.lista {
            if cancion.artista == artista {
                canciones_del_artista.push(cancion);
            }
        }
        canciones_del_artista
    }
    //     ➔ modificar título de la playlist.
    fn modificar_titulo(&mut self, titulo: String) {
        self.nombre_de_playlist = titulo;
        self.guardar_en_archivo();
    }
    //     ➔ eliminar todas las canciones.x1
    fn eliminar_playlist(&mut self) {
        self.lista.clear();
        self.guardar_en_archivo();
    }
    fn guardar_en_archivo(&self) {
        let msg = "No se pudo guardar en el archivo";
        let json = serde_json::to_string(&self.lista).expect(msg);
        let mut file = fs::File::create("src/tp5/playlist.json ").expect(msg);
        file.write_all(json.as_bytes()).expect(msg)
    }
    fn cargar_de_archivo(&mut self) {
        let msg = "No se pudo cargar el archivo";
        let data = fs::read_to_string("src/tp5/playlist.json").expect(msg);
        self.lista = serde_json::from_str(&data).expect(msg);
    }
}
pub fn main() {}

#[test]
fn test_agregar_cancion() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    let cancion = Cancion::new(
        String::from("Bohemian Rhapsody"),
        String::from("Queen"),
        Genero::Rock,
    );
    playlist.agregar_cancion(cancion);
    assert_eq!(playlist.lista.len(), 1);
    let cancion = Cancion::new(
        //vuelvo a crear otra cancion con los mismos valores, para comparar, asi no hago clone
        String::from("Bohemian Rhapsody"),
        String::from("Queen"),
        Genero::Rock,
    );
    assert_eq!(playlist.lista[0], cancion);
}

#[test]
fn test_eliminar_cancion() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    let cancion = Cancion::new(
        String::from("Bohemian Rhapsody"),
        String::from("Queen"),
        Genero::Rock,
    );
    playlist.agregar_cancion(cancion);
    playlist.eliminar_cancion();
    assert_eq!(playlist.lista.len(), 0);
}

#[test]
fn test_mover_cancion() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    let cancion1 = Cancion::new(String::from("pep"), String::from("p"), Genero::Rock);
    let cancion2 = Cancion::new(
        String::from("pepe1"),
        String::from("pepeartosta"),
        Genero::Pop,
    );
    playlist.agregar_cancion(cancion1);
    playlist.agregar_cancion(cancion2);
    let cancion_movida = Cancion::new(
        String::from("movd"),
        String::from("sarasamov"),
        Genero::Rock,
    );
    playlist.mover_cancion_pos(cancion_movida, 1); //si es 1, esta en la pos 0, si le mando 0 esta en la pos 0
    let cancion_movida = Cancion::new(
        String::from("movd"),
        String::from("sarasamov"),
        Genero::Rock,
    );
    assert_eq!(playlist.lista[0], cancion_movida); //
}

#[test]
fn test_buscar_cancion_por_nombre() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    let cancion1 = Cancion::new(String::from("pep"), String::from("p"), Genero::Rock);
    let cancion2 = Cancion::new(
        String::from("pepe1"),
        String::from("pepeartosta"),
        Genero::Pop,
    );
    playlist.agregar_cancion(cancion1);
    playlist.agregar_cancion(cancion2);
    let cancion = playlist.buscar_cancion_por_nombre(String::from("pepe1"));
    assert_eq!(cancion.unwrap().titulo, "pepe1");
}
#[test]
fn test_obter_canciones_de_un_determinado_genero() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    let cancion1 = Cancion::new(String::from("pep"), String::from("p"), Genero::Rock);
    let cancion2 = Cancion::new(
        String::from("pepe1"),
        String::from("pepeartosta"),
        Genero::Pop,
    );
    playlist.agregar_cancion(cancion1);
    playlist.agregar_cancion(cancion2);
    let canciones = playlist.obter_canciones_de_un_determinado_genero(Genero::Pop);
    assert_eq!(canciones.len(), 1);
    assert_eq!(canciones[0].titulo, "pepe1");
}
#[test]
fn test_obtener_canciones_de_un_determinado_artista() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    let cancion1 = Cancion::new(String::from("pep"), String::from("p"), Genero::Rock);
    let cancion2 = Cancion::new(
        String::from("pepe1"),
        String::from("pepeartosta"),
        Genero::Pop,
    );
    playlist.agregar_cancion(cancion1);
    playlist.agregar_cancion(cancion2);
    let canciones = playlist.obtener_canciones_de_un_determinado_artista("p");
    assert_eq!(canciones.len(), 1);
    assert_eq!(canciones[0].titulo, "pep");
}
#[test]
fn test_modificar_titulo() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    playlist.modificar_titulo(String::from("Mi Playlist Modificada"));
    assert_eq!(playlist.nombre_de_playlist, "Mi Playlist Modificada");
}
#[test]
fn test_eliminar_playlist() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    let cancion1 = Cancion::new(String::from("pep"), String::from("p"), Genero::Rock);
    let cancion2 = Cancion::new(
        String::from("pepe1"),
        String::from("pepeartosta"),
        Genero::Pop,
    );
    playlist.agregar_cancion(cancion1);
    playlist.agregar_cancion(cancion2);
    playlist.eliminar_playlist();
    assert_eq!(playlist.lista.len(), 0);
}
#[test]
fn test_playlist() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    let cancion1 = Cancion::new(String::from("pep"), String::from("p"), Genero::Rock);
    let cancion2 = Cancion::new(
        String::from("pepe1"),
        String::from("pepeartosta"),
        Genero::Pop,
    );
    playlist.agregar_cancion(cancion1);
    playlist.agregar_cancion(cancion2);
    let cancion = playlist.buscar_cancion_por_nombre(String::from("pepe1"));
    assert_eq!(cancion.unwrap().titulo, "pepe1");
    let canciones = playlist.obter_canciones_de_un_determinado_genero(Genero::Pop);
    assert_eq!(canciones.len(), 1);
    assert_eq!(canciones[0].titulo, "pepe1");
    let canciones = playlist.obtener_canciones_de_un_determinado_artista("p");
    assert_eq!(canciones.len(), 1);
    assert_eq!(canciones[0].titulo, "pep");
    playlist.modificar_titulo(String::from("Mi Playlist Modificada"));
    assert_eq!(playlist.nombre_de_playlist, "Mi Playlist Modificada");
    playlist.eliminar_playlist();
    assert_eq!(playlist.lista.len(), 0);
}
#[test]
fn test_guardar_y_cargar_playlist() {
    //de forma aislada, anda joya, tengo un problema por usar el archivo en varios tests
    // eliminar archivo json si existe para que no haya problemas
    let _ = fs::remove_file("src/tp5/playlist.json");
    
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    let cancion1 = Cancion::new(
        String::from("Bohemian Rhapsody"),
        String::from("Queen"),
        Genero::Rock,
    );
    let cancion2 = Cancion::new(
        String::from("Imagine"),
        String::from("John Lennon"),
        Genero::Pop,
    );

    playlist.agregar_cancion(cancion1);
    playlist.agregar_cancion(cancion2);
    playlist.guardar_en_archivo();

    // Crear una nueva instancia de la playlist y cargar los datos desde el archivo
    let mut nueva_playlist = Playlist::new(String::from("Nueva Playlist"), Vec::new());
    nueva_playlist.cargar_de_archivo();

    assert_eq!(nueva_playlist.lista.len(), 2);
    assert_eq!(nueva_playlist.lista[0].titulo, "Bohemian Rhapsody");
    assert_eq!(nueva_playlist.lista[0].artista, "Queen");
    assert_eq!(nueva_playlist.lista[1].titulo, "Imagine");
    assert_eq!(nueva_playlist.lista[1].artista, "John Lennon");
}
#[test]
fn test_cargar_playlist_vacia() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"), Vec::new());
    playlist.guardar_en_archivo();

    // Crear una nueva instancia de la playlist y cargar los datos desde el archivo
    let mut nueva_playlist = Playlist::new(String::from("Nueva Playlist"), Vec::new());
    nueva_playlist.cargar_de_archivo();

    assert_eq!(nueva_playlist.lista.len(), 0);
}

//html
// cargo tarpaulin --target-dir src/coverage --skip-clean --exclude-files=target/debug/* --out html
