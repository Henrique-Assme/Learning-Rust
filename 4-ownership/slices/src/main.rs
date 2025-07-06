fn main() {
    let mut s = String::from("hello world");
    let hello = &s[0..5]; // fatia da string de 0 a 4
    let world = &s[6..11]; // outra fatia da string de 6 a 10
    // hello e world são slices, basicamente são referências a partes da string original
    // possuem um ponteiro para o início da fatia e um tamanho 
    let word1 = first_word(&s);
    let word2 = corrected_first_word(&s);
    s.clear();
    // como word2 é uma fatia da string original, é uma referência imutável. Ao tentar limpar a string, estamos tenando mutar ela
    // mas word2 ainda está referenciando a string original, o que causa um erro de compilação
    println!("hello: {}, world: {}, word1: {}, word2: {}", hello, world, word1, word2);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // retorn o índice do final da primeira palavra
        }
    }
    s.len()
}

// Essa implementação não é boa, pois ela retorna o índice da primeira palavra, mas se a string for limpa depois de chamar a função, o índice não será mais válido.

fn corrected_first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // retorna a fatia da string até o espaço
        }
    }
    &s[..] // se não houver espaço, retorna a string inteira
}