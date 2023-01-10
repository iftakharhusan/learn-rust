// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


fn main() {
    let drinka = Drink {
        flavor: Flavors::Orange,
        quantity_mg: 500
    };
    let drinkb = Drink {
        flavor: Flavors::Mango,
        quantity_mg: 300
    };
    let drinkc = Drink {
        flavor: Flavors::Apple,
        quantity_mg: 1000
    };
    let drinkd = Drink {
        flavor: Flavors::Strawberry,
        quantity_mg: 600
    };

    print_drink(&drinka);
    print_drink(&drinkb);
    print_drink(&drinkc);
    print_drink(&drinkd);
}

#[derive(Debug)]
enum Flavors {
    Orange,
    Mango,
    Strawberry,
    Apple
}

struct Drink {
    flavor: Flavors,
    quantity_mg: i32
}

fn print_drink(drink: &Drink){
    println!("Drink(flavor=\"{:?}\", quantity=\"{:?}ml\")", drink.flavor, drink.quantity_mg);
}
