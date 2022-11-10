# Exercice 2 - Guess the number

_Le but du jeu est de trouver un nombre généré aléatoirement entre 1 et 100. Pour ce faire, le joueur propose un nombre et le jeu répond si c'est plus, moins ou bien le bon nombre._

- Le code se situe dans `src/main.rs`
- Le fichier `Cargo.toml` sert de fichier de configration
- Pour compiler `main.rs` il vous suffit d'éxécuter la commande `cargo build` à la racine du projet.<br>
- Pour run votre code, il suffit d'éxécuter la commande `cargo run` au même endroit que le build

## Quelques petits tips qui pourront vous être utile

> Pour lire dans la console il faut utiliser :
> ```Rust
> println!("Veuillez saisir votre phrase");
> let mut input = String::new();
> 
> io::stdin()
>     .read_line(&mut input)
>     .expect("Échec de la lecture de l'entrée utilisateur");
> ``` 
> La variable `input` contiendra la valeur de l'input utilisateur

> Pour transformer une entrée utilisateur en int il faut utiliser :
>```Rust
>let number: u32 = match input.trim().parse() {
>   Ok(nombre) => nombre,
>   Err(_) => continue,
>};
>```
>La variable `number` contiendra l'input en type `integer`

>L'opérateur `loop` permet de créer un boucle infinie à l'instar de `while true`, pour quitter cette boucle il suffit d'appeler l'opérateur `break`