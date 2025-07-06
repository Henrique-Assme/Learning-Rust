fn main() {
    let s = String::from("Hello, world!");
    takes_ownership(s);
    // println!("{}", s);

    let x = 6;
    makes_copy(x);
    println!("{}", x);

    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let s3 = String::from("Hello, Rust!");
    let len2 = calculate_length_ref(&s3);
    println!("The length of '{}' is {}.", s3, len2);

    let mut s4 = String::from("Hello");
    change(&mut s4);
    println!("{}", s4);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// Quando passamos uma variável para uma função, ela é movida para dentro da função.
// Isso significa que a variável original não pode mais ser usada após a chamada da função.
// Se quisermos usar a variável original após a chamada da função, precisamos passar uma referência para ela.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// Tipos primitivos como i32 são copiados, então podemos usar a variável original após a chamada da função.
// Isso é chamado de "cópia".

// Fazer a função assim não é algo que é recomenado, para isso podemos fazer a passagem por referência para que a
// variável original possa ser usada após a chamada da função e não precise ser retornada como uma tupla

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

// Passar por referência é o que Rust chama de "borrow".
// Isso significa que estamos emprestando a variável para a função, mas não estamos movendo
// a propriedade dela. A variável original ainda pode ser usada após a chamada da função.
// Isso é feito com o símbolo & antes do tipo da variável na assinatura da função.
// Referências por padrão são imutáveis, mas podemos fazer referências mutáveis
// com o símbolo &mut antes do tipo da variável na assinatura da função.

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// No entando, para garantir a segurança do código, Rust não permite que tenhamos múltiplas referências mutáveis
// ou uma referência mutável e uma referência imutável ao mesmo tempo.
// Isso é chamado de "aliasing" e pode causar problemas de concorrência.
// Rust garante que não teremos problemas de concorrência ao garantir que não teremos múltiplas referências
// mutáveis ou uma referência mutável e uma referência imutável ao mesmo tempo.
// Isso é feito com o sistema de tipos do Rust e o compilador verifica isso em tempo de compilação.
// Se tentarmos fazer isso, o compilador nos dará um erro.

fn more_than_one_mutable_reference() {
    let mut s1 = String::from("Hello");
    let s2 = &s1; // Isso é permitido, pois é uma referência imutável
    let s3 = &mut s1; // Isso não é permitido, pois estamos tentando ter uma referência mutável e uma referência imutável ao mesmo tempo
    // let s4 = &mut s1; // Isso também não é permitido, pois estamos tentando ter múltiplas referências mutáveis ao mesmo tempo
    println!("{}", s2); 
    println!("{}", s3);
    // println!("{}", s4);
}