mod front_of_house; // declarando o módulo front_of_house, que está no arquivo front_of_house.rs

pub use crate::front_of_house::hosting;
// podemos usar o use para trazer o módulo hosting para o escopo atual, assim podemos chamar a função add_to_waitlist diretamente
// sem precisar usar o prefixo front_of_house::hosting::add_to_waitlist()
// o pub use permite que outros módulos também acessem hosting diretamente sem estar no arquivo lib.rs

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    // como o módulo hosting está no escopo diretamente usando 'use', podemos chamar a função diretamente sem o prefixo front_of_house::
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    } 

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }       
    }

    pub enum Appetizer{
        Soup,
        Salad,
    }
}