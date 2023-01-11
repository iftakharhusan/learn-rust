// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
#[derive(Debug)]
enum Ticket{
    Standard(f64),
    Backstage(f64, String),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Standard(50 as f64),
        Ticket::Backstage(100 as f64, "Iftakhar".to_owned()),
        Ticket::Vip(500.88 as f64, "Aisha".to_owned()),
        Ticket::Vip(1000 as f64, "Ayan".to_owned())
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("STD: ${}", price),
            Ticket::Backstage(price, owner) => println!("BKS {}: ${}", owner, price),
            Ticket::Vip(price, owner) => println!("VIP {}: ${}", owner, price)
        }
    }
    
}
