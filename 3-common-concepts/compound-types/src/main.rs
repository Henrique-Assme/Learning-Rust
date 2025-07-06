fn main() {
    // compound types
    // são tipos que podem conter mais de um valor
    // tuplas - poddem ser pensados como arrays de tamanho fixo que podem conter valores de tipos diferentes
    let tup = ("Henrique", 22);
    // para acessar os valores de uma tupla, usamos a notação de ponto ou desestruturação
    let (name, age) = tup;
    let name = tup.0; // acessando o primeiro elemento
    let age = tup.1; // acessando o segundo elemento
    println!("Name: {}, Age: {}", name, age);

    // arrays - são coleções de valores do mesmo tipo, com tamanho fixo
    // rrrays tem tamanho fixo, para tamanhos dinâmicos, usamos vetores (vectors)
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1]; // acessando o segundo elemento
    let x = error_codes[4]; // acessando o quinto elemento, isso vai causar um panic, pois o array tem apenas 3 elementos
    // Rust previne que você acesse um índice fora do tamanho do array, isso é uma das garantias de segurança de memória que Rust oferece

    let byte = [0; 8]; // array de 8 bytes, todos com valor 0, outra forma de inicializar um array

}
