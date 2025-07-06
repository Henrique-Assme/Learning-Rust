fn main() {
    // regras de ownsership
    // 1 - Cada valor em Rust tem uma variável que é o seu dono.
    // 2 - Somente pode haver um dono por vez.
    // 3 - Quando o dono sai de escopo, o valor será descartado.
    // regras de referência
    // 1 - Referências são imutáveis por padrão.
    // 2 - Podemos fazer referências mutáveis com o símbolo &mut.
    // 3 - Não podemos ter referências mutáveis e imutáveis ao mesmo tempo.
    // 4 - Não podemos ter múltiplas referências mutáveis ao mesmo tempo.

    { // s não é válido aqui, pois não foi declarado ainda
        let s = "hello"; // s é válido a partir daqui
        // s aqui é uma string literal e é armazenado diretamente em binário e na stack
        let s_string = String::from("hello"); // s_string é uma String, que é armazenada no heap
        // em outras linguagens seria usada a notação new e delete para adicionar e remover memória da heap
        // Rust faz isso automaticamente com o sistema de ownership
    } // s não é mais válido aqui, pois saiu de escopo

    copy_ownership();
    clone();
}


fn copy_ownership() {
    let x = 5; // x é uma variável inteira, que é copiada
    let y = x; // y agora também é 5, mas x ainda é 5
    println!("x: {}, y: {}", x, y); // ambos são válidos

    let s1 = String::from("hello"); // s1 é uma String, que é movida
    let s2 = s1; // s2 agora é o dono da String, s1 não é mais válido, valor de s1 foi movido para s2 
    // println!("s1: {}", s1); // isso causaria um erro de compilação, pois s1 não é mais válido
    println!("s2: {}", s2); // s2 é válido e imprime "hello"
}

fn clone() {
    let s1 = String::from("hello"); // s1 é uma String
    let s2 = s1.clone(); // s2 é uma cópia de s1, ambos são válidos
    println!("s1: {}, s2: {}", s1, s2); // ambos imprimem "hello"
    // clone cria uma cópia profunda do valor, enquanto a atribuição normal move o valor
    // isso é útil quando você precisa manter o valor original e também ter uma cópia dele
    // mas clone pode ser mais custoso em termos de desempenho, pois envolve alocação
    // de memória adicional e cópia de dados, enquanto a atribuição normal é mais eficiente
    // porque não envolve cópia de dados, apenas a transferência de propriedade
    // é importante entender quando usar clone para evitar problemas de desempenho
    // e garantir que você não está movendo valores desnecessariamente
}