// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

fn main() {
    let item = Item {
        id: 3232,
        quantity: 50
    };
    
    show_id(&item);
    show_quantity(&item);
}


struct Item {
    id: i32,
    quantity: i32
}

fn show_id(item: &Item){
    println!("id={}", item.id)
}
fn show_quantity(item: &Item){
    println!("quantity={}", item.quantity)
}