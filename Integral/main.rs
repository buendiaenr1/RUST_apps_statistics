// evaluar una expresion matematica
use std::collections::BTreeMap;
use fasteval::Evaler;    // use this trait so we can call eval().
use fasteval::Compiler;  // use this trait so we can call compile().
// leer un cadena (expresion matematica)

// Enrique buendia Lozada sep/2020
// BUAP




fn main() -> Result<(), fasteval::Error> {
    // evaluar expresion en string
    let parser = fasteval::Parser::new();
    let mut slab = fasteval::Slab::new();
    let mut map = BTreeMap::new();
    // leer la expresiona evaluar
    println!("");
    println!("");
    println!("Integral usando regla trapezoidal");
    println!("Enrique RP Buendia Lozada BUAP:2020");
    let mut line = String::new();
    println!("f(x) =");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    let compiled = parser.parse(&line, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);

    // leer los limites de integral
    let mut a=String::new();
    println!("límite inferior  a= ");
    let _b1 = std::io::stdin().read_line(&mut a).unwrap();
    let a_f: f64 = a.trim().parse().expect("por favor corrija!");

    let mut b=String::new();
    println!("límite superior  b= ");
    let _b1 = std::io::stdin().read_line(&mut b).unwrap();
    let b_f: f64 = b.trim().parse().expect("por favor corrija!");

    let mut n=String::new();
    println!("número de intervalos a usar para aproximación  n= ");
    let _b1 = std::io::stdin().read_line(&mut n).unwrap();
    let n_int: i32 = n.trim().parse().expect("por favor corrija!");

    let mut vec1:Vec<f64>=Vec::new();
    let inx=(b_f-a_f)/n_int as f64;
    let mut x=a_f+0.0;

    for  i in 0..n_int {
        map.insert("x".to_string(), x as f64);
        let mut val = fasteval::eval_compiled!(compiled, &slab, &mut map);
        eprintln!("para {} = {}", x, val);
        if i==0 {
            val=val*0.5;
            vec1.push(val);
        }else{
            vec1.push(val);
        }
        x=x+inx;
        
    }

    x=b_f+0.0;
    map.insert("x".to_string(), x as f64);
    let mut val = fasteval::eval_compiled!(compiled, &slab, &mut map);
    eprintln!("para {} = {}", x, val);
    val=val*0.5;
    vec1.push(val);
    println!("{:?}", vec1); // mostrar vector

    // suma del vector
    let sum: f64 = vec1.iter().sum();
    let integral: f64 =sum*inx;
    println!("La suma es: {} \n La integral es: {}", sum,integral);

    println!("\n \n");
    // ************ evaluación de la expresion matemática
    //let x=90;
    //map.insert("x".to_string(), x as f64);
    //let val = fasteval::eval_compiled!(compiled, &slab, &mut map);
    //eprintln!("para {} = {}", x, val);

    
    // leer algo para terminar
    let mut n=String::new();
    println!("Oprima enter: ");
    let _b1 = std::io::stdin().read_line(&mut n).unwrap();
    //let n_int: i32 = n.trim().parse().expect("por favor corrija!");
    
    Ok(())
}