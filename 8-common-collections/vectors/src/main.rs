fn main() {
    let a = [1,2,3];
    // a é um array de 3 elementos do tipo i32 que tem tamanho fixo e não pode ser alterado
    let mut v: Vec<i32> = Vec::new();
    // v é um vetor de tamanho dinâmico que pode crescer ou encolher conforme necessário
    v.push(1);
    v.push(2);
    v.push(3);

    {
        let v2 = vec![1,2,3]; 
        // v2 é um vetor inicializado com os valores 1, 2 e 3
        // como vector não é um tipo primitivo, ele é alocado na heap
        // e não na stack como os arrays
        // o vetor v2 é automaticamente desalocado quando sai do escopo
    }
    // print!("v: {:?}\n", v2);

    // let third = &v[20]; // isso causa um erro em tempo de execução, pois como o vetor é armazenado na heap, não há garantia de que o índice 20 exista
    // para evitar esse erro, podemos usar o método get
    match v.get(2) {
        Some(third) => println!("third: {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("i: {}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("Integer: {}", i),
        SpreadsheetCell::Float(f) => println!("Float: {}", f),
        SpreadsheetCell::Text(s) => println!("Text: {}", s),
        _ => println!("Not a valid cell type"),
    }
}
