extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number =  rand::thread_rng().gen_range(1,101);
    // println!("El numero secreto es: {}",secret_number);
    println!("Adivina el numero");

    loop {
        println!("Por favor, introduce tu adivinanza: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Fallo al leer la linea");

        let guess:u32 = match guess.trim().parse(){
        // .expect("Porfavor, ingrese un numero!");
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("Tu Adivinanza: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!(" -> (Numero bajo)\n"),
            Ordering::Greater => println!(" ->(Numero alto)\n"),
            Ordering::Equal => {
                println!("\nGANESTE!");
                break;
            }
        }
    }
}
