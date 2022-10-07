pub struct Caixa {
    pub saldo: f64,
    pub saida: f64,
    pub entrada: f64
}

impl Caixa {
    pub fn adicionar_saida(&mut self, valor: f64) {
        self.saldo -= valor;
        self.saida += valor;
    }
    pub fn adicionar_entrada(&mut self, valor: f64) {
        self.saldo += valor;
        self.entrada += valor;
    }
    pub fn dados_gerais(&self) -> String {
        format!(
            "O seu saldo e de {}, total de entradas é {} e total de saida é {}",
            self.saldo.to_string(),
            self.entrada.to_string(),
            self.saida.to_string()
        )
    }
}