use std::io::{self, Write};

use chrono::NaiveDate;

// Recebe uma mensagem para printar e aguarda a entrada do usuário.
// Retorna a entrada do usuário convertida para o tipo genérico T.
// O tipo genérico T necessita ter a implementação do FromStr.
// Funcional para os tipos i32, u32, f32, f64, String.
pub fn get_input<T: std::str::FromStr>(msg: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();

    let first_letter_of_type = std::any::type_name::<T>().chars().next().unwrap();
    let separador_local = get_separador();
    let separador_milhares = if separador_local == '.' { ',' } else { '.' };

    loop {
        println!("{}", msg);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        // if (i)nt, (u)int, (f)loat
        if (first_letter_of_type == 'i'
            || first_letter_of_type == 'u'
            || first_letter_of_type == 'f')
        {
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    // Remove espaços em branco e separadores de milhares e testa se o resultado é uma string vazia
                    if input.trim().replace(separador_milhares, "") == "" {
                        println!("Entrada inválida. Tente novamente.\n");
                        continue;
                    } else {
                        // Não sendo vazia, realiza o parse
                        match input.trim().parse::<T>() {
                            Ok(value) => return value,
                            Err(_) => {
                                println!("Entrada inválida. Tente novamente.\n");
                                continue;
                            }
                        }
                    }
                }
                Err(_) => {
                    println!("Entrada inválida. Tente novamente.\n");
                    continue;
                }
            }
        }

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Remove espaços em branco e separadores de milhares e testa se o resultado é uma string vazia
                if input.trim().replace(separador_milhares, "") == "" {
                    println!("Entrada inválida. Tente novamente.\n");
                    continue;
                } else {
                    // Não sendo vazia, realiza o parse
                    match input.trim().parse::<T>() {
                        Ok(value) => return value,
                        Err(_) => {
                            println!("Entrada inválida. Tente novamente.\n");
                            continue;
                        }
                    }
                }
            }
            Err(_) => {
                println!("Entrada inválida. Tente novamente.\n");
                continue;
            }
        }
    }
}

fn get_separador() -> char {
    let i = 123;
    let f = i as f32 / 100 as f32;
    let separador = f.to_string().chars().nth(1).unwrap();
    return separador;
}

/**
 * Recebe uma mensagem para printar e aguarda a entrada do usuário.
 * A entrada deve respeitar o padrão de string no formato dd#mm#aaaa, onde # é um separador qualquer, exceto dígitos e letras.
 *
 */
pub fn get_date_input(msg: &str) -> NaiveDate {
    let mut date = String::new();
    loop {
        date = get_input(msg);
        if date.len() != 10 {
            println!("Data inválida. Use o padrão dd-mm-aaaa. Exemplo: 31-01-1900.\n");
            continue;
        }

        let split_pattern: char = date.chars().nth(2).unwrap();
        if split_pattern.is_digit(10) {
            println!(
                "Data inválida. Utilize um separador não alfanumérico, respeitando o formato dd-mm-aaaa. Exemplo: 31-01-1900\n"
            );
            continue;
        }

        let mut split_date: Vec<&str> = date.split(split_pattern).collect::<Vec<&str>>();
        if split_date.len() != 3 {
            println!("Data inválida. Utilize um separador não alfanumérico, respeitando o formato dd-mm-aaaa. Exemplo: 31-01-1900\n");
            continue;
        }

        let dia: u16 = match split_date.get(0).unwrap().parse::<u16>() {
            Ok(v) => v,
            Err(_) => {
                println!("Data inválida. Dias, meses e anos precisam ser valores numéricos.\n");
                continue;
            }
        };
        let mes: u16 = match split_date.get(1).unwrap().parse::<u16>() {
            Ok(v) => v,
            Err(_) => {
                println!("Data inválida. Dias, meses e anos precisam ser valores numéricos.\n");
                continue;
            }
        };
        let ano: u16 = match split_date.get(2).unwrap().parse::<u16>() {
            Ok(v) => v,
            Err(_) => {
                println!("Data inválida. Dias, meses e anos precisam ser valores numéricos.\n");
                continue;
            }
        };

        if ano < 0 || ano > 9999 {
            println!("Data inválida. O ano precisa ser um valor entre 0 e 9999.\n");
            continue;
        }

        if ((ano % 4 == 0) && (ano % 100 != 0 || ano % 400 == 0)) && mes == 2 && dia > 29 {
            println!("Data inválida. Fevereiro possui 29 dias em anos bissextos.\n");
            continue;
        }

        if dia < 1 || dia > 31 {
            println!("Data inválida. O dia precisa ser um valor entre 1 e 31.\n");
            continue;
        }

        if mes == 2 && dia > 28 {
            println!("Data inválida. O fevereiro possui apenas 28 dias.\n");
            continue;
        }

        if (mes == 4 || mes == 6 || mes == 9 || mes == 11) && dia > 30 {
            println!("Data inválida. O mês {} possui apenas 30 dias.\n", mes);
            continue;
        }

        if mes < 1 || mes > 12 {
            println!("Data inválida. O mês precisa ser um valor entre 1 e 12.\n");
            continue;
        }

        return NaiveDate::from_ymd_opt(ano as i32, mes as u32, dia as u32).unwrap();
    }
}

/**
 * Recebe uma mensagem para printar e aguarda a entrada do usuário.
 * Irá forçar que a entrada seja uma cadeia de caracteres com no máximo max_length caracteres.
 */
pub fn get_str_with_max_length(msg: &str, max_length: usize) -> String {
    let mut input = String::new();
    loop {
        input = get_input(msg);
        if input.len() > max_length {
            println!(
                "Entrada inválida. O texto precisa ter no máximo {} caracteres.\n",
                max_length
            );
            continue;
        }
        return input;
    }
}

/**
 * Recebe uma mensagem para printar e aguarda a entrada do usuário.
 * Irá forçar que a entrada seja um valor inteiro entre min e max.
 * Aceita valores negativos na range de i32 e retorna o valor convertido para i32, i64 ou i128
 */
pub fn get_int_in_range<T: std::convert::From<i32>>(msg: &str, min: i32, max: i32) -> T {
    let mut input: i32 = 0;
    loop {
        input = get_input(msg);
        if input < min || input > max {
            println!(
                "Entrada inválida. O valor precisa ser entre {} e {}.\n",
                min, max
            );
            continue;
        }
        return input.into();
    }
}

/**
 * Recebe uma mensagem para printar e aguarda a entrada do usuário.
 * Irá forçar que a entrada seja um valor inteiro sem sinal entre min e max.
 * A valores positivos na range de u32 e retorna o valor convertido para u32, u64 ou u128
 */
pub fn get_uint_in_range<T: std::convert::From<u32>>(msg: &str, min: u32, max: u32) -> T {
    let mut input: u32 = 0;
    loop {
        input = get_input(msg);
        if input < min || input > max {
            println!(
                "Entrada inválida. O valor precisa ser entre {} e {}.\n",
                min, max
            );
            continue;
        }
        return input.into();
    }
}
