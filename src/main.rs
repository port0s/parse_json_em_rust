use serde_derive::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Endereco {
    rua: String,
    bairro: String,
    numero: u32,
    complemento: String
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Dados {
    nome: String,
    idade: Option<u32>,
    endereco: Endereco,
    fones: Vec<String>
}

fn main() {
    let json_str = std::include_str!("../src/dados.json");

    let res: Dados = serde_json::from_str(json_str).unwrap();

    println!("Olá, {}", res.nome);


    match res.idade {
        Some(x) => println!("{:?} anos",  x),
        None => println!("idade não informada")
    }

    dbg!("{}, res");
}
