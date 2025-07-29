use std::fs::{self, File};
use std::io::{ErrorKind, Read};
use std::io;

fn main() {
    // panic();
    // a();
    // Assim como options, o enum Result também é uma enumeração que pode ser usada para lidar com erros de forma segura.
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    let f = File::open("hello.txt");
    // O Result é um enum que pode ser usado para lidar com erros de forma segura.
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
    // Podemos também criar o arquivo caso ele não exista, usando o match para lidar com o erro de forma mais específica.
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
            other_error => panic!("Problem opening the file: {:?}", other_error), 
        }
    };
    // Outra maneira de fazer isso sem usar match é usando o método unwrap_or_else, que recebe uma closure para lidar com o erro.
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {:?}", e);
            }) 
            // Esse é um statement, pois não retorna um valor, apenas executa uma ação
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Podemos também usar o método unwrap, que irá retornar o valor se for Ok, ou panic se for Err
    let f = File::open("hello.txt").unwrap();
    // Ou o método expect, que permite personalizar a mensagem de erro
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Basicamente o panic! é uma macro que interrompe a execução do programa
fn panic() {
    panic!("Crash and burn");
}

// Exemplo de panic para usar RUST_BACKTRACE=1 cargo run
fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}

// Error propagation é o processo de passar erros de uma função para outra, permitindo que a função chamadora lide com o erro de forma apropriada.


fn read_username_from_file() -> Result<String, io::Error>{
    let mut f = File::open("hello.txt")?;
    // O operador ? é uma forma de lidar com erros de forma mais concisa, ele irá retornar o erro caso a operação falhe, ou o valor Ok caso a operação seja bem suceda.
    // O operador ? é usado para propagar erros

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    f.read_to_string(&mut s)?;
    Ok(s)

    // Podemos simplificar ainda mais a função fazendo chain calls
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // podemos ainda tornar essa função mais simplificada usando o método read_to_string diretamente na chamada do File::open
    // fs::read_to_string("hello.txt")
}

// Como a função main é especial, ela não retorna nada ou retorna um Result, para poder usar o operador ? dentro dela, precisamos usar a macro main com o tipo Result.
// fn main() -> Result<(), io::Error> {
//     let username = read_username_from_file()?;
//     println!("Username: {}", username);
//     Ok(())
// }

// Idealmente usamos o Result enum e error propagation para lidar com erros de forma segura e eficiente, evitando o uso de panic! em produção, pois isso pode causar a interrupção inesperada do programa.
// O uso de panic! é mais adequado para situações onde o programa não pode continuar, como em casos de falhas de lógica ou condições inesperadas que não podem ser tratadas de forma segura