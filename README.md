# PROYECTO-KMEANS-RUST

K-Means (clustering, aprendizaje no supervisado) implementado **desde cero en Rust**.
Proyecto para aprender clustering y Rust al mismo tiempo, sin usar librerías de ML.

## Estructura

```
PROYECTO-KMEANS-RUST/
├── Cargo.toml
└── src/
    ├── main.rs     # Genera datos, ejecuta K-Means e imprime resultados
    ├── punto.rs    # struct Punto + distancia euclidiana
    ├── datos.rs    # Generador de datos sintéticos (3 grupos)
    └── kmeans.rs   # Algoritmo K-Means
```

## Cómo ejecutarlo

```bash
cargo run
```

## Hoja de ruta (busca los `TODO` en el código)

- [ ] **TODO 1** `punto.rs`: distancia euclidiana
- [ ] **TODO 2** `datos.rs`: generar puntos con ruido alrededor de cada centro
- [ ] **TODO 3** `kmeans.rs`: inicializar centroides al azar
- [ ] **TODO 4** `kmeans.rs`: asignar cada punto al centroide más cercano
- [ ] **TODO 5** `kmeans.rs`: recalcular centroides (promedio)
- [ ] **TODO 6** `kmeans.rs`: bucle principal hasta converger

## Extensiones

- [ ] Leer datos desde un CSV (crate `csv`)
- [ ] Visualizar clusters en SVG/PNG (crate `plotters`)
- [ ] Método del codo para elegir k
- [ ] K-Means++ para mejor inicialización
- [ ] Soporte para más dimensiones (`Vec<f64>`)
- [ ] Comparar con DBSCAN

## Qué se practica en Rust

structs e `impl`, `Vec` e iteradores, referencias (`&`, `&mut`), módulos, manejo de `f64` y el crate `rand`.
