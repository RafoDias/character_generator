use crate::{
    controller::character_controller::{get_by_name, save},
    models::{character::Character, Class},
    utils::cli::cli_validation::{
        get_date_input, get_input, get_int_in_range, get_str_with_max_length,
    },
};

pub fn start() {
    println!("\nOlá! Bem-vindo ao gerador de personagens.");

    let mut acao = menu_acoes();
    while acao != 0 {
        acao = menu_acoes();
    }
    println!("Fechando...");
    return;
}

fn criar_personagem() {
    let name: String =
        get_str_with_max_length("Digite o nome do personagem (1 a 40 caracteres):", 40);

    let mut class_question = "Escolha a classe do personagem:".to_string();

    for class in Class::iter() {
        class_question.push_str(&format!("\n{} - {}", class as i32, class));
    }

    let class: Class = get_int_in_range(&class_question, 1, 3);

    let level: i32 = get_int_in_range("Digite o nível do personagem (De 0 a 100):", 0, 100);

    let birthday: chrono::NaiveDate =
        get_date_input("Digite a data de nascimento do personagem (dd-mm-aaaa):");

    let character: Character = crate::Character {
        name,
        class,
        level,
        birthday,
    };
    let returned_character: Character = save(&character);

    println!("Personagem salvo.\n{:?}", returned_character);
}

fn buscar_personagem() {
    let name: String =
        get_str_with_max_length("Digite o nome do personagem (1 a 40 caracteres):", 40);

    let returned_character: Character = get_by_name(&name);

    if returned_character.name == "" {
        println!("Personagem não encontrado.");
        return;
    }

    println!("Personagem encontrado:\n{:?}", returned_character);
}

fn remover_personagem() {
    let name: String =
        get_str_with_max_length("Digite o nome do personagem (1 a 40 caracteres):", 40);

    let character: Character = crate::Character {
        name,
        class: Class::NoClass,
        level: 0,
        birthday: chrono::NaiveDate::from_ymd_opt(1, 1, 1).unwrap(),
    };

    let returned_character = crate::character_controller::remove(&character);

    if returned_character.name == "" {
        println!("Personagem não encontrado.");
        return;
    } else {
    }
    println!("Personagem removido:\n{:?}", returned_character);
}

fn menu_acoes() -> i32 {
    let mut int_input: i32 = 0;

    int_input = get_input(
        "\nO que deseja fazer?
        1 - Criar um personagem
        2 - Buscar um personagens
        3 - Remover um personagem
        0 - Sair",
    );
    println!("Você escolheu a opção {}", int_input);

    match int_input {
        1 => criar_personagem(),
        2 => buscar_personagem(),
        3 => remover_personagem(),
        0 => return 0,
        _ => {
            println!("Opção inválida");
            return 0;
        }
    }

    return int_input;
}
