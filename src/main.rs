#[derive(Debug)] // this allows us to print using {:?}
struct Animal {
    name: String,
    quantity: u32,
    condition: String,
}

fn main() {
    // Start a program, create a fine empty basket
    let mut animal_basket: Vec<Animal> = Vec::new();

    // Add new animals
    add_animal(&mut animal_basket, "Pig", 20, "Healthy");
    add_animal(&mut animal_basket, "Camel", 30, "Calm");
    add_animal(&mut animal_basket, "Donkey", 14, "Strong");

    // Loop through the animals, so we can print them
    for animal in &animal_basket {
        println!(
            "We have {} {}()s and they are {}.",
            animal.quantity, animal.name, animal.condition
        );
    }

    // Developer style print, print the basket using RUST debug feature. Decided to add this cause it's part of what I learn in this chapter
    println!("\nFull Animal Basket(Debug View): {:?}", animal_basket);
}

// Create a function to add new animals
fn add_animal(basket: &mut Vec<Animal>, name: &str, quantity: u32, condition: &str) {
    let new_animal = Animal {
        name: name.to_string(),
        quantity,
        condition: condition.to_string(),
    };
    basket.push(new_animal);
}
