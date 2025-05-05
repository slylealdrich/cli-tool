mod animal;
mod animal_type;
mod data;

use animal::Animal;
use animal_type::AnimalType;
use clap::{Parser, Subcommand};
use data::Data;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    View(AnimalCommand),
    Buy(AnimalCommand),
    Sell(AnimalCommand),
}

#[derive(Parser, Debug)]
struct AnimalCommand {
    #[arg(value_enum)]
    animal: AnimalType,
    name: Option<String>,
}

fn main() {
    // Setup the data

    let data = data::Data::new();

    match data.init_db() {
        Ok(_) => (),
        Err(error) => eprintln!("Error initializing DB {}", error),
    }

    // Setup the CLI

    let args = Args::parse();

    match &args.command {
        Command::View(command) => view(data, &command.animal, &command.name),
        Command::Buy(command) => buy(data, &command.animal, &command.name),
        Command::Sell(command) => sell(data, &command.animal, &command.name),
    }
}

fn view(data: Data, animal_type: &AnimalType, name: &Option<String>) {
    match name {
        Some(name) => {
            let animal = data.select_animal_by_name(&name).unwrap();

            match animal {
                Some(animal) => print_animal(&animal),
                None => println!("DOES NOT EXIST"),
            }
        }
        None => {
            let animals = data
                .select_animals_by_animal_type(&animal_type.to_string())
                .unwrap();

            match animals.is_empty() {
                true => println!("You have no {}", animal_type),
                false => {
                    animals.iter().for_each(|animal| {
                        print_animal(&animal);
                    });
                },
            }

            animals.is_empty();
        }
    }
}

fn buy(data: Data, animal_type: &AnimalType, name: &Option<String>) {
    match name {
        None => println!("Here's all the available {:?}", animal_type),
        Some(name) => match data.insert_animal(animal_type, name) {
            Ok(_) => println!("Bought the {:?} named {:?}", animal_type, name),
            Err(error) => eprintln!("{}", error),
        },
    }
}

fn sell(data: Data, animal_type: &AnimalType, name: &Option<String>) {
    match name {
        None => println!("Here's all your {:?} with their prices", animal_type),
        Some(name) => match data.delete_animal_by_name(name) {
            Ok(_) => println!("Sold the {:?} named {:?}", animal_type, name),
            Err(error) => eprintln!("{}", error),
        },
    }
}

// View functions

fn print_animal(animal: &Animal) {
    println!(
        " L...L     \n< o o >    \n \\   /     \n  ^_^  {:?} | {:?}",
        animal.name, animal.animal_type
    );
}
