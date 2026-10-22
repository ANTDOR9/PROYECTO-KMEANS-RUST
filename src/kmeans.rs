use crate::punto::Punto;

/// Resultado final del algoritmo.
#[derive(Debug)]
pub struct Resultado {
    /// Posición final de cada centroide (largo = k).
    pub centroides: Vec<Punto>,
    /// `asignaciones[i]` = índice del cluster al que pertenece `puntos[i]`.
    pub asignaciones: Vec<usize>,
    /// Cuántas iteraciones tardó en converger.
    pub iteraciones: usize,
}

/// Algoritmo K-Means completo.
///
/// Pasos:
/// 1. Elegir k centroides iniciales.
/// 2. Asignar cada punto al centroide más cercano.
/// 3. Recalcular cada centroide como el promedio de sus puntos.
/// 4. Repetir 2 y 3 hasta que las asignaciones ya no cambien (o hasta `max_iter`).
pub fn kmeans(puntos: &[Punto], k: usize, max_iter: usize) -> Resultado {
    // TODO 6: unir las funciones de abajo en el bucle principal.
    todo!("bucle principal de K-Means")
}

/// Elige k centroides iniciales tomando k puntos distintos al azar de los datos.
/// Pista: `rand::seq::index::sample(&mut rng, puntos.len(), k)`.
fn inicializar_centroides(puntos: &[Punto], k: usize) -> Vec<Punto> {
    // TODO 3
    todo!("inicializar centroides")
}

/// Para cada punto, devuelve el índice del centroide más cercano.
/// Pista: `iter().enumerate().min_by(|a, b| a.1.partial_cmp(b.1).unwrap())`
/// sobre las distancias (f64 no implementa `Ord`, por eso `partial_cmp`).
fn asignar_clusters(puntos: &[Punto], centroides: &[Punto]) -> Vec<usize> {
    // TODO 4
    todo!("asignar clusters")
}

/// Nuevo centroide de cada cluster = promedio (x, y) de sus puntos.
/// Ojo con los clusters vacíos: decide qué hacer (p. ej. conservar el centroide anterior).
fn recalcular_centroides(puntos: &[Punto], asignaciones: &[usize], k: usize) -> Vec<Punto> {
    // TODO 5
    todo!("recalcular centroides")
}
