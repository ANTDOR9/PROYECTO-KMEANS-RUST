/// Un punto en el plano 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Punto {
    pub x: f64,
    pub y: f64,
}

impl Punto {
    pub fn new(x: f64, y: f64) -> Self {
        Punto { x, y }
    }

    /// Distancia euclidiana entre `self` y `otro`.
    ///
    /// Fórmula: sqrt((x1 - x2)^2 + (y1 - y2)^2)
    /// Pista: f64 tiene los métodos `.powi(2)` y `.sqrt()`.
    pub fn distancia(&self, otro: &Punto) -> f64 {
        // TODO 1: implementar la distancia euclidiana
        todo!("calcular distancia euclidiana")
    }
}
