extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    
    println!("o número secreto é: {}", numero_secreto);

    loop {
        println!("digite o seu palpite: ");
        let mut palpite = String::new();
        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
        println!("Você disse: {}", palpite);
        match palpite.cmp(&numero_secreto){
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Você acertou!!!");
                break;
            }   
        }
        
    }
}


// fn main() {
//     println!("Advinhe um número");
//     println!("Digite seu palpite: ");
//     let mut palpite = String::new();
//     io::stdin().read_line(&mut palpite)
//         .expect(" Falha ao ler entrada ");
//     println!("Você disse: {}", palpite)
// }
