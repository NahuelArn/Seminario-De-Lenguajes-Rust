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
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

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

    fn agregar_cancion(&mut self, cancion: Cancion) {
        self.lista.push(cancion);
    }
    fn eliminar_cancion(&mut self) {
        self.lista.pop();
    }
}
pub fn main() {}
