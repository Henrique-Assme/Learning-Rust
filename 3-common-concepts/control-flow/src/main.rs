fn main() {
    // control flow
    let number = 5;

    if number < 10 {
        println!("First condition was true");
    } else if number < 20 {
        println!("Second condition was true");
    } else {        
        println!("No conditions were true");
    }

    // a expressão necessariamente deve ser um booleano, diferente de outras linguagens
    // podemos ter if else dentro de let
    let condition = true;
    let _number = if condition { 5 } else { 6 };

    // diferentes tipos de loop
    // loop executa o código até encontrar um break ou return
    loop {
        println!("Again and again");
        break; // para sair do loop
    }
    let mut counter  = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter; // para sair do loop e retornar um valor
        }
    };
    println!("The result is {result}");

    // while loop - executa enquanto a condição for verdadeira
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    // for loop - itera sobre uma coleção
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {element}");
    }

    for number in 1..4 { // 1..4 é um range de 1 a 3
        println!("{number}!");
    }
}
