Primeiro criei um arquivo main.rs para fazer um primeiro hello world em rust. Isso foi feito na função main() com a "função" println!() para imprimir o hello world.

No entando, essa maneira de escrever código em Rust não é o ideal para grandes projetos, para isso surge o cargo, um package manager de rust que permite criar projetos e executar tudo através dele.

Para comparação, no primeiro caso fiz a criação do main.rs e da função main, foi compilado com ```rustc main.rs``` e executado com ./main
No segundo caso foi criado o projeto com ```cargo new hello-cargo```. Isso criou uma pasta com alguns arquivos
-hello-cargo
    -src
        -main.rs
    -.gitignore
    -Cargo.toml (equivalente a um package.json)
O main.rs criado já possuia o println! com hello world. Para compilar usei ```cargo build``` dentro da pasta e isso gerou alguns outros arquivos, como o próprio executável, o Cargo.lock (equivalente ao package-lock.json) para especificar exatamente como o código foi compilado e uma pasta target com diversas outras coisas.
Depois disso, ```cargo run``` roda o código efetivamente.
O comando ```cargo check``` é bom para verificar erros no código sem gerar um executável dele.