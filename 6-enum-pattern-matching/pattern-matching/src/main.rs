fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(10);
    match some_value {
        Some(10) => println!("Ten!"),
        _ => (),
    }

    if let Some(10) = some_value {
        println!("Ten!");
    }

    // ao usar o if let, o padrão é verificado e, se corresponder, a variável é vinculada ao valor dentro do Some.
    // Se não corresponder, nada acontece. Isso é útil quando você só se importa com
    // um caso específico e não precisa lidar com todos os outros casos possíveis.
    // O if let é uma maneira concisa de lidar com enumerações e outras estruturas
    // que podem ter múltiplos valores, permitindo que você se concentre apenas no caso
    // que realmente lhe interessa, sem a necessidade de escrever um padrão de correspondência completo.
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}