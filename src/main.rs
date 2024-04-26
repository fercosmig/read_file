use std::fs::File;
use std::io::prelude::*;

fn main()
{
    let mut arquivo = File::create("teste.txt").expect("arquivo: File::create error");

    arquivo.write_all(b"Arquivo de testes sendo criado!").expect("arquivo: write_all error");

    {
        let mut arquivo = File::open("teste.txt").expect("arquivo: File::open error");
        let mut conteudo = String::new();

        arquivo.read_to_string(&mut conteudo).expect("arquivo: read_to_string error");
        println!("Conteúdo do arquivo: \n\n{}", conteudo);
    }
}
