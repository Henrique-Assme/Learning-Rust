fn main() {
    my_function();
    function_with_parameters(5, 10);
    let result = sum(3, 4);
    println!("Sum: {}", result);
    let result_with_return = sum_with_return(10, 20);
    println!("Sum with return: {}", result_with_return);
}

fn my_function() {
    println!("Another function");
}

fn function_with_parameters(param1: i32, param2: i32) {
    println!("Parameters: {}, {}", param1, param2);
}

// podemos pensar em Rust que um código é um statement ou expression
// statement: é uma linha de código que executa uma ação, mas não retorna um valor
// expression: é uma linha de código que retorna um valor, e pode ser usada em outros

// ambas as funções acima são statements, pois não retornam valores

fn sum(a: i32, b: i32) -> i32 {
    a + b // esta é uma expressão, pois retorna um valor
} // ela retorna pois a última linha de uma função é sempre o valor retornado e sem ;

// podemos retornar valores com ou sem o uso de return
fn sum_with_return(a: i32, b: i32) -> i32 {
    return a + b; // esta também é uma expressão, mas usa o return
}