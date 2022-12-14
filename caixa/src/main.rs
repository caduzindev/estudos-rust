use std::io;
use crate::financeiro::fluxo::Caixa;

pub mod financeiro;

fn main() {
    println!("Inicie o seu caixa");

    let entrada = io_input();

    let entrada: f64 = entrada.parse().unwrap();

    let mut caixa = Caixa {
        entrada,
        saldo: entrada,
        saida: 0.0
    };

    loop {
        println!("1: Adicionar saida");
        println!("2: Adicionar entrada");
        println!("3: Ver dados gerais");
        println!("q: sair");

        let escolha = io_input();

        if escolha == "1" {
            println!("Valor da saida:");

            let saida = io_input();

            let saida: f64 = saida.parse().unwrap();

            caixa.adicionar_saida(saida);
        }

        if escolha == "2" {
            println!("Valor da entrada:");

            let entrada = io_input();

            let entrada: f64 = entrada.parse().unwrap();

            caixa.adicionar_entrada(entrada);
        }

        if escolha == "3" {
            println!("{}", caixa.dados_gerais());
        }

        if escolha == "q" {
            break;
        }
    }
}

fn io_input() -> String {
    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler a linha");

    String::from(entrada.trim())
}