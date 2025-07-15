fn main() {
    // Rust não possui um tipo null, mas podemos usar enums para representar a ausência de valor.
    // O enum Option<T> é usado para representar um valor que pode ou não estar presente.
    // Ele tem duas variantes: Some(T) e None.
    // Um dado que pode ou não estar presente pode ser wraped no option enum
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
    // Isso não compila porque Option<i8> não pode ser somado diretamente a i32.
    // Para resolver isso, precisamos extrair o valor de y usando pattern matching
    // E lidar com cada variante do enum Option<T>. Podemos fazer isso o método unwrap()

    let sum = x + y.unwrap_or(0);
    // Desse modo, como x e y são números, eles podem ser somados
    // Se y for None, o unwrap_or(0) retornará 0, evitando um panic.
}
