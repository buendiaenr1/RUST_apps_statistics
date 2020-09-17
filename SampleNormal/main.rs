// para generar la  muestra de mediciones simulada
// con distribución normal
use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

// para escribir los datos en un archivo
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


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

    // escribir a un archivo
    
    let path = Path::new("muestra_simulada.txt");
    let display = path.display();
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("no puedo crear...{}: {}", display, why),
        Ok(file) => file,
    };


    let mut vec1: Vec<f64> = Vec::new(); // vector de la muestra creada
    let mut suma = 0.0;

    for i in 0..n_i {
        let mut rng = thread_rng();
        let normal = Normal::new(media_f, ds_f)?;
        let v = normal.sample(&mut rng);
        println!("elemento {}  = {} ", i + 1, v);
        file.write_all((v.to_string()).as_bytes());
        suma = suma + v;
        vec1.push(v);
    }
    let med = suma / n_i as f64;
    println!("muestra creada : \n {:?}", vec1); // mostrar vector
    println!("Promedio de muestra creada = { }", med);
    file.write_all("\n Media \n".as_bytes());
    file.write_all((med.to_string()).as_bytes());
    let mut sc = 0.0;
    for i in vec1.iter() {
        sc = sc + (i - med).powf(2.0);
    }
    if n_i <= 30 { 
        let dsc = (sc / (n_i as f64 - 1.0)).powf(0.5);
        println!("Desviación estandar (m) de la muestra creada = { }", dsc);
        file.write_all("\n ds_p \n".as_bytes());
        file.write_all((dsc.to_string()).as_bytes());
    }else{
        let dsc = (sc / (n_i as f64)).powf(0.5);
        println!("Desviación estandar (p) de la muestra creada = { }", dsc);
        file.write_all("\n ds_p \n".as_bytes());
        file.write_all((dsc.to_string()).as_bytes());
    }


    // leer algo para terminar
    let mut n = String::new();
    println!("Oprima enter: ");
    let _b1 = std::io::stdin().read_line(&mut n).unwrap();

    Ok(())
}