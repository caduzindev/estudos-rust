use std::io;

struct Caixa {
    saldo: f64,
    saida: f64,
    entrada: f64
}

impl Caixa {
    fn adicionar_saida(&mut self, valor: f64) {
        self.saldo -= valor;
        self.saida += valor;
    }
    fn adicionar_entrada(&mut self, valor: f64) {
        self.saldo += valor;
        self.entrada += valor;
    }
    fn dados_gerais(&self) -> String {
        format!(
            "O seu saldo e de {}, total de entradas é {} e total de saida é {}",
            self.saldo.to_string(),
            self.entrada.to_string(),
            self.saida.to_string()
        )
    }
}

fn main() {
    println!("Inicie o seu caixa");

    let mut entrada = String::new();

    io_input(&mut entrada);

    let entrada: f64 = entrada.trim().parse().unwrap();

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

        let mut escolha = String::new();

        io_input(&mut escolha);

        let escolha = escolha.trim();

        if escolha == "1" {
            println!("Valor da saida:");

            let mut saida = String::new();

            io_input(&mut saida);

            let saida: f64 = saida.trim().parse().unwrap();

            caixa.adicionar_saida(saida);
        }

        if escolha == "2" {
            println!("Valor da entrada:");

            let mut entrada = String::new();

            io_input(&mut entrada);

            let entrada: f64 = entrada.trim().parse().unwrap();

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

fn io_input(input: &mut String) {
    io::stdin()
        .read_line(input)
        .expect("Falha ao ler a linha");
}