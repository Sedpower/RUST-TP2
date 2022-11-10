use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devinez le nombre !");

    //TODO 1 - générer un nombre aléatoire entre 1 et 100 et le stocker dans la variable "nombre_secret"
    let nombre_secret = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Veuillez entrer un nombre.");
        let mut supposition = String::new();

        //TODO 2 - Récupérer la saisie du joueur dans la variable supposition
        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        //TODO 3 - Transformer la saisie en un int utilisable pour le comparer avec nombre_secret
        let bombre_saisi: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        //TODO 4 - Comparer nombre_saisi avec "nombre_secret" et print dans le terminal si c'est plus moins ou gagné 
        match bombre_saisi.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}