use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}

fn start() {
    let mut input = String::new();
    println!("Olá! Bem-vindo ao gerador de personagens.");
    println!("O que deseja fazer?");
    println!("1 - Criar um personagem");
    println!("2 - Buscar um personagens");
    println!("0 - Sair");
    input = read_line();

    println!("Você escolheu a opção {}", input);
}

fn criar_personagem() {
    println!("Digite o nome do personagem:");
    let name = read_line();

    println!("Escolha a classe do personagem:");
    println!("1 - Guerreiro");
    println!("2 - Ladino");
    println!("3 - Clérigo");
    let class = read_line();

    println!("Digite o nível do personagem:");
    let level = read_line();

    println!("Digite a data de nascimento do personagem (aaaa-mm-dd):");
    let birthday = read_line();
}

fn buscar_personagem() {
    println!("Digite o nome do personagem:");
    let name = read_line();
}
