enum IpAddrKind {
    V4(String),
    V6(u8, u8, u8, u8, u8, u8, u8, u8),
}

// enum são tipos de dados que podem ser usados para definir um tipo com um número fixo de variantes.
// Cada variante pode ter dados associados a ela, como V4 que tem um `String` associado a ela.

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // usamos o operador `::` para acessar as variantes do enum
    let localhost = IpAddr {
        kind: four(String::from("")),
        address: String::from("127.0.0.1")
    };

    let localhost = IpAddrKind::V4(String::from("127.0.0.1")); // podemos usar o enum diretamente para criar uma instância
    let localhost = IpAddrKind::V6(127, 0, 0, 1, 0, 0, 0, 1); // podemos usar a variante V6 com os dados associados
    let msg = Message::some_function();
}

// podemos usar enums em funções
fn route(_ip_kind: IpAddrKind) {}

// enum pode ser usado para armazenar vários tipos de dados diferentes
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// poderiamos definir cada variante como uma struct, mas a vantagem do enum é que ele agrupa todas as variantes em um único tipo

impl Message {
    fn some_function() {
        println!("This is a function inside an enum");
    }
}