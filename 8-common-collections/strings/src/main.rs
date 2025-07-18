use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Strings are stored as collections of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "initial contents"; //string slice
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    // como UTF-8 é uma codificação de caracteres, podemos armazenar strings em diferentes idiomas
    let hello = String::from("Hello");
    let hello_in_chinese = String::from("你好");
    let hello_in_arabic = String::from("مرحبا");
    let hello_in_russian = String::from("Привет");
    let hello_in_japanese = String::from("こんにちは");

    let mut s5 = String::from("foo");
    s5.push_str("bar"); // push_str adiciona uma string slice ao final de uma String existente
    s5.push('!'); // push adiciona um único caractere ao final de uma String existente
 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2); // format! cria uma nova String sem mover as originais
    let s3 = s1 + &s2; // s1 é movido, s2 permanece válido

    let hello = String::from("Hello");
    // let c = hello[0]; // Isso não compila, pois Rust não permite acessar bytes diretamente de uma String
    // isso acontece porque um caractere pode ocupar mais de um byte em UTF-8
    for c in hello.bytes() {
        println!("{}", c); // imprime os bytes da string
    }
    for c in hello.chars() {
        println!("{}", c); // imprime os caracteres da string
    }
    // para o grapheme cluster, que é a representação visual de um caractere, usamos o crate unicode-segmentation
    for c in hello.graphemes(true) {
        println!("{}", c); // imprime os grapheme clusters da string
    }
}
