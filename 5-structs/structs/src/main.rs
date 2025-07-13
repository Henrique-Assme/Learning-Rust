struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Structs são úteis para agrupar dados relacionados.
// Eles podem conter diferentes tipos de dados e são usados para criar tipos compostos.
// No entanto, uma vez que um campo de uma struct é definido como imutável,
// não é possível alterar o valor desse campo diretamente.

#[derive(Debug)] // Derivando o trait Debug para permitir a impressão do struct
// isso permite que o compilador gere automaticamente a implementação do trait Debug para o struct,
// o que é útil para depuração e impressão de valores de structs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectangle {
    // Método associado para criar um retângulo quadrado
    // Métodos associados são funções que são chamadas diretamente no tipo, sem precisar de uma instância
    // Eles são definidos dentro de um bloco `impl` e podem ser usados para criar
    // instâncias do tipo ou fornecer funcionalidades relacionadas ao tipo.
    // O `self` não é necessário aqui, pois não estamos usando uma instância do struct
    // mas sim criando uma nova instância do struct.
    // O `self` é usado para referenciar a instância atual do struct, mas
    // neste caso, estamos criando uma nova instância do struct `Rectangle` sem precisar de uma instância existente.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // podemos tornar o struct mutável usando a palavra-chave `mut`
    // e podemos alterar os valores dos campos, desde que o campo em si seja mutável

    let mut user1 = User {
        email: String::from("heniassme@gmail.com"),
        username: String::from("Henrique Assme"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username =  String::from("Henrique Assme 2");

    let user2 = build_user(
        String::from("brunakmotta@gmail.com"), 
        String::from("Bruna Kostruba")
    );

    let user3 = User {
        email: String::from("outro usuário"),
        username: String::from("Outro Usuário"),
        ..user2 // Usando a sintaxe de atualização de struct para copiar os campos de user2
    };

    // Structs também podem ser definidos como tuplas, sem especificar nomes de campos
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // Mesmo possuindo os mesmos tipos de dados, structs tuplas são diferentes

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(&rect)
    ); 

    // Ao tentar imprimir o struct diretamente, ele não implementa o trait `Display`, um trait padrão do Rust
    // que permite a formatação de saída. Para imprimir
    // println!("rect: {}", rect);
    // precisamos implementar o trait `Debug` para o struct para poder imprimir com `{:?}` ou podemos colocar o #[derive(Debug)] acima do struct
    // ou podemos implementar o trait `Display` manualmente, mas isso é mais complexo.
    // Para imprimir o struct com o trait Debug, usamos `{:?}` ou `{:#?}` para uma formatação mais legível
    println!("rect: {:#?}", rect);

    // Usando a implementação do método `area` do struct Rectangle
    println!(
        "The area of the rectangle using the method is {} square pixels.",
        rect.area()
    );

    // Verificando se um retângulo pode conter outro retângulo
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    if rect1.can_hold(&rect2) {
        println!("rect1 can hold rect2");
    } else {
        println!("rect1 cannot hold rect2");
    }

    let square = Rectangle::square(10);
    println!("Square: {:#?}", square);
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Função para calcular a área de um retângulo, mas não é parte do struct
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}