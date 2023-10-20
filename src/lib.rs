use std::{io::Write, str::FromStr};

pub fn read_input_and_parse<T: FromStr>(prompt: &str) -> T {
    loop {
        print!("{prompt}");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)
            .expect("Erro ao ler a entrada");
        match line.trim().parse::<T>() {
            Ok(num) => break num,
            Err(_) => eprintln!("ERRO: Por favor introduza um valor válido")
        }
    }
}
