// Exercise 3: Traits and Structs

mod duck;

// This is required!!!!
use duck::Duck;

fn main() {
    let mut donald = duck::FlightlessDuck {
        name: "Donald".to_string(),
        swimming: false,
    };
    let roger = duck::FlightfulDuck {
        name: "Roger".to_string(),
        swimming: false,
        flying: false,
    };
    println!("{:?}", donald);
    println!("{}", donald);

    donald.start_swiming().expect("Error trying to swim.");

    println!("{:?}", donald);
    println!("{:?}", roger);

    for d in get_ducks().iter() {
        println!("{}", d);
    }

    make_it_quack(&roger);
    make_it_quack(&donald);
    donald.start_swiming().expect("Error trying to swim.");
}

/// Gets a Vec of Ducks
/// Box is needed because we might not know the size of Duck at compile time
/// dyn is needed to access a Dynamic Trait
fn get_ducks() -> Vec<Box<dyn Duck>> {
    vec![
        Box::new(duck::FlightlessDuck {
            name: "Donald".to_string(),
            swimming: false,
        }),
        Box::new(duck::FlightfulDuck {
            name: "Roger".to_string(),
            swimming: false,
            flying: false
        }),
    ]
}

/// Takes in a reference to a dynamic Duck so we don't need a Box!
fn make_it_quack(d: &dyn Duck) {
    println!("{}", d.quack());
}

// Won't work because Duck must be dynamic to access the functions by just the trait
// fn get_random_duck_2() -> Vec<Duck> {
//     vec![duck::FlightlessDuck { name: "Donald".to_string(), swimming: false }, duck::FlightfulDuck { name: "Roger".to_string(), swimming: false }]
// }

// Won't work because the compiler doesn't know how large a dyn Duck is!
// fn get_random_duck_3() -> Vec<dyn Duck> {
//     vec![duck::FlightlessDuck { name: "Donald".to_string(), swimming: false }, duck::FlightfulDuck { name: "Roger".to_string(), swimming: false }]
// }
