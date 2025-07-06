fn main() {
    // variáveis em rust são imutáveis por padrão
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // Isso causará um erro de compilação, pois x é imutável
    println!("The value of x is: {}", x);

    // Para tornar uma variável mutável, usamos a palavra-chave 'mut'
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The value of y is now: {}", y);

    // rust possui o conceito de valores constantes, você pode definir uma constante usando a palavra-chave 'const'
    const PI: f32 = 3.14;
    println!("The value of PI is: {}", PI);
    // o padrão de nomeação de constantes é usar letras maiúsculas com underscores caso tenha mais de uma palavra
    // Constantes não podem ser mutáveis através da palavra-chave 'mut'
    // constantes precisam ter um tipo explícito, diferente do x ou y anteriormente
    // constantes precisam ser definidas como valores constantes, uma constante não pode receber o resultado de uma função
    // rust permite colocar underscore para facilitar a leitura em valores grandes como 123_456_789

    let z = 10;
    println!("The value of z is: {}", z);
    let z = "twenty";
    println!("The value of z is now: {}", z);
    // Isso é chamado de "shadowing", onde você pode redefinir uma variável com o mesmo nome
    // Isso é útil quando você precisa alterar o tipo de uma variável ou redefinir seu valor
}
