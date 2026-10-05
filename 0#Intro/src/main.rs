use std::io;
use rand::RngExt;

fn main() {
    println!("Devinez le nombre !");

    let secret = rand::rng().random_range(1..11);

    println!("Veuillez entrer un nombre:");

    let mut supposition: String = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Echec de la lecture de l'entrée utilisateur");

    match supposition.parse::<i32>() {
        Ok(nombre) => verification(nombre, secret),
        Err(erreur) => println!("Un erreur s'est produite lors de la convertion. Erreur={}", erreur),
    }
}

fn verification(nombre: i32, secret: i32) {
    if nombre == secret {
        println!("Vous avez trouvé ! Le nombre était bien {}", secret);
    } else {
        println!("Raté ! Le nombre était {}", secret);
    }
}