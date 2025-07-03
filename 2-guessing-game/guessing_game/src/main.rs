use std::io; // Importa o módulo io da biblioteca padrão
use rand::Rng; // Importa o módulo Rng da biblioteca rand, que é usada para gerar números aleatórios
// para poder usar a biblioteca rand, é necessário adicioná-la ao arquivo Cargo.toml
// Adicione a seguinte linha ao arquivo Cargo.toml:
// [dependencies]
// rand = "0.8" (ou a versão mais recente disponível)
// Depois de adicionar a dependência, execute o comando `cargo build` para baixar e compilar a biblioteca
use std::cmp::Ordering; // Importa o módulo Ordering do std::compare, que é usado para comparar valores
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // thread_rng() cria um gerador de números aleatórios baseado na thread atual
    // gen_range(1, 101) gera um número aleatório entre 1 e 100 (limite exclusivo)
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
        // String é um tipo de dado em rust que representa uma sequência de caracteres
        // Ela não é a string primitiva, mas sim uma estrutura de dados que permite manipular strings de forma mais flexível
        // new cria uma nova string vazia
        // Em rust variáveis são imutáveis por padrão, então para tornar uma variável mutável, usamos a palavra-chave mut
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        // stdin() retorna a entrada padrão do programa, que é o teclado
        // read_line lê uma linha da entrada padrão e a armazena na variável guess
        // &mut guess passa uma referência mutável da variável guess para a função read_line para que seja possível modificar o valor dela
        // expect é um método que trata o resultado da leitura, se houver um erro, ele imprime a mensagem "Failed to read line" e encerra o programa
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // trim() remove espaços em branco no início e no final da string
        // parse() tenta converter a string em um número do tipo u32 (unsigned 32-bit integer)
        // expect() é um método que trata o resultado da conversão, se houver
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    
        // Ordering é um enum que representa o resultado da comparação entre dois valores
        // cmp é um método que compara dois valores e retorna um Ordering
        // match é uma estrutura de controle que permite executar diferentes blocos de código dependendo do resultado
    }
    // loop repete o códgo que estiver nele. Nesse caso o break dentro do match que determina a parada


}
