/*
- Defina la estructura Cancion con campos para el título, el artista y el género. El género
puede ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta
por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre
ella:
➔ agregar canción.
➔ eliminar canción.
➔ mover canción // mueve la canción a una determinada posición de la playlist.
➔ buscar canción por nombre.
➔ obtener las canciones de un determinado género.
➔ obtener las canciones de un determinado artista.
➔ modificar título de la playlist.
➔ eliminar todas las canciones.x1


*/
// #[derive(PartialEq)]

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

#[derive(Debug)]
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
#[derive(Debug)]
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
    }
    //➔ eliminar canción.
    fn eliminar_cancion(&mut self) {
        self.lista.pop();
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
    }
    //     ➔ eliminar todas las canciones.x1
    fn eliminar_playlist(&mut self) {
        self.lista.clear();
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
    playlist.mover_cancion_pos(cancion_movida, 1);  //si es 1, esta en la pos 0, si le mando 0 esta en la pos 0
    let cancion_movida = Cancion::new(
        String::from("movd"),
        String::from("sarasamov"),
        Genero::Rock,
    );
    assert_eq!(playlist.lista[0], cancion_movida);  //
}
