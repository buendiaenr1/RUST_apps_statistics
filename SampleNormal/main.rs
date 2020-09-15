use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

// Enrique RP Buendia Lozada
// BUAP 2020

fn main() -> Result<(), NormalError> {
    println!("");
    println!("");
    println!("Crear una muestra normal usando: n, media y desv.estadnd");
    println!("Enrique RP Buendia Lozada BUAP:2020 ");
    // leer media de la muestra
    let mut n = String::new();
    println!("Tamaño de la muestra  n = ");
    let _b1 = std::io::stdin().read_line(&mut n).unwrap();
    let n_i: i32 = n.trim().parse().expect("por favor corrija...");
    // leer media de la muestra
    let mut media = String::new();
    println!("Media = ");
    let _b1 = std::io::stdin().read_line(&mut media).unwrap();
    let media_f: f64 = media.trim().parse().expect("por favor corrija...");
    // leer desviación estandar de la muestra
    let mut ds = String::new();
    println!("Desviación estándar = ");
    let _b1 = std::io::stdin().read_line(&mut ds).unwrap();
    let ds_f: f64 = ds.trim().parse().expect("por favor corrija...");

    let mut vec1: Vec<f64> = Vec::new(); // vector de la muestra creada
    let mut suma = 0.0;
    for i in 0..n_i {
        let mut rng = thread_rng();
        let normal = Normal::new(media_f, ds_f)?;
        let v = normal.sample(&mut rng);
        println!("elemento {}  = {} ", i + 1, v);
        suma = suma + v;
        vec1.push(v);
    }
    let med = suma / n_i as f64;
    println!("muestra creada : \n {:?}", vec1); // mostrar vector
    println!("Promedio de muestra creada = { }", med);

    let mut sc = 0.0;
    for i in vec1.iter() {
        sc = sc + (i - med).powf(2.0);
    }
    let dsc = (sc / (n_i as f64 - 1.0)).powf(0.5) / n_i as f64;
    println!("Desviación estandar de la muestra creada = { }", dsc);

    // leer algo para terminar
    let mut n = String::new();
    println!("Oprima enter: ");
    let _b1 = std::io::stdin().read_line(&mut n).unwrap();

    Ok(())
}
