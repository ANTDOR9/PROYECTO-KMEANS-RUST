mod datos;
mod kmeans;
mod punto;

use punto::Punto;

fn main() {
    // Tres centros "reales" alrededor de los cuales se generan los datos.
    let centros_reales = [
        Punto::new(2.0, 2.0),
        Punto::new(8.0, 3.0),
        Punto::new(5.0, 9.0),
    ];

    let puntos = datos::generar_datos(&centros_reales, 50, 1.5);
    println!("Puntos generados: {}", puntos.len());

    let resultado = kmeans::kmeans(&puntos, 3, 100);

    println!("Convergió en {} iteraciones", resultado.iteraciones);
    for (i, c) in resultado.centroides.iter().enumerate() {
        let cantidad = resultado.asignaciones.iter().filter(|&&a| a == i).count();
        println!("Cluster {i}: centroide = ({:.2}, {:.2}), puntos = {cantidad}", c.x, c.y);
    }
}
