Todo o código feito até agora foi em apenas um arquivo main.rs, mas isso não é o que acontece em projetos maiores
Precisamos de uma maneira de organizar e encapsular o código também, apenas expondo aquilo que desejamos

Rust tem um sistema de módulos, como quando criamos um package com o cargo new, por exemplo, esse pacote armazena crates como executáveis ou códigos que podem ser usados por outros programas
Crates por sua vez armazenam módulos

Quando criamos um package com cargo new my-project, podemos ver no Cargo.toml que é um package e ele possui uma crate binária, por mais que ela não apareça
Essa crate existe porque existe um arquivo main.rs no diretório src e isso por padrão cria uma crate binária cujo root é esse arquivo main.rs
Da mesma forma se um lib.rs existe no src, Rust cria automaticamente outra crate de biblioteca com root sendo lib.rs 
Um package sempre tem ao menos uma crate, além de conter 0 ou 1 library crate
Pode ter qualquer número de crates binárias

Podemos criar uma lib como default colocando a flag --lib ao criar um novo package com o cargo

Dentro de um módulo, as chields são sempre privadas na perspectiva dos parents, ou seja, mas um chield modulo pode ver tudo que foi definido dentro do parent module
Podemos também usar super keyword para ter acesso a função de fora do módulo, que podem ser criadas fora dele

Por padrão, tudo é privado, o struct criado dentro do módulo é privado, funções, campos de um struct, mesmo que o struct tenha pub, seus campos continuam sendo privados