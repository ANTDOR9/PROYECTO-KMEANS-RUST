use rand::Rng;

use crate::punto::Punto;

/// Genera datos sintéticos: `puntos_por_grupo` puntos alrededor de cada centro.
///
/// Cada punto = centro + ruido aleatorio en x e y, con ruido en [-dispersion, dispersion].
/// Pista: `let mut rng = rand::thread_rng();` y `rng.gen_range(-dispersion..dispersion)`.
pub fn generar_datos(centros: &[Punto], puntos_por_grupo: usize, dispersion: f64) -> Vec<Punto> {
    // TODO 2: recorrer cada centro, crear `puntos_por_grupo` puntos con ruido
    // y juntarlos todos en un solo Vec<Punto>.
    todo!("generar datos sintéticos")
}
