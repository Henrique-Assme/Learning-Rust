fn main() {
    // Rust tem 4 tipos de dados primitivos principais os scalaer types, que representam um único valor, e os compound types, que podem agrupar múltiplos valores.
    // Os tipos escalares são:
    // integers - números inteiros, que podem ser signed (com sinal) ou unsigned (sem sinal), e podem ter diferentes tamanhos (8, 16, 32, 64 bits, ou o tamanho de um ponteiro)
    let a = 98_222; // decimal, padrão sem anotar o tipo é i32
    let b = 0xff; // hex
    let c = 0o77; // octal
    let d = 0b1111_0000; // binary
    let e = b'A'; // byte (u8)
    let f: u8 = 255; // unsigned 8-bit integer
    // ao tentar atribuir 256 para f, causará um erro de compilação, pois o valor excede o limite do tipo u8 em builds de debug
    // em build release, o valor será truncado para 0, 257 será truncado para 1, e assim por diante em complemento de 2
    
    // floating point numbers
    let f = 2.0; // f64, padrão sem anotar o tipo é f64
    let g: f32 = 3.0;

    // addition, subtraction, multiplication, division, and remainder
    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let remainder = 43 % 5; // remainder

    // booleans - representa um valor verdadeiro ou falso
    let t = true; // boolean true
    let f = false; // boolean false
    
    // character - representa um único caractere Unicode, que pode ser um emoji, letra, símbolo, etc.
    let c = 'z'; // character
    let z = '😻'; // emoji character
    
}
