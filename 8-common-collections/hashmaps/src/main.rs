use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50); // os nomes das equipes não são enviados por referência, mas sim por valor

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25); // atualiza o valor da chave "Blue"
    scores.entry(String::from("Yellow")).or_insert(50); // or_insert não altera o valor se a chave já existir, mas insere se não existir

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // a coleção das palavras é ["hello", "world", "wonderful", "world"]
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // essa linha sempre retorna uma referência para o valor associado à chave `word`
        // se a chave não existir, insere 0 e retorna uma referência para esse valor
        // se a chave já existir, retorna uma referência para o valor existente
        // o valor é do tipo &mut i32, então podemos usar *count para acessar
        // o valor e incrementá-lo
        *count += 1; // incrementa o contador para cada palavra
    }
    
    println!("{:?}", map); // imprime o mapa com as contagens de palavras)
}
