use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devinez le nombre !");

    //TODO 1 - générer un nombre aléatoire entre 1 et 100 et le stocker dans la variable "nombre_secret"

    loop {
        println!("Veuillez entrer un nombre.");
        let mut supposition = String::new();

        //TODO 2 - Récupérer la saisie du joueur dans la variable supposition

        //TODO 3 - Transformer la saisie en un int "nombre_saisi" utilisable pour le comparer avec "nombre_secret"

        //TODO 4 - Comparer nombre_saisi avec "nombre_secret" et print dans le terminal si c'est plus moins ou gagné
    }
}