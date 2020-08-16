use std::convert::TryFrom;
use std::io;

mod game;
mod parser;

use game::{Entity, Scene};
use parser::{EntityIdent, EntityToken, Instruction};

fn find_entity(scene: &mut Scene, ident: EntityIdent) -> Result<&mut Entity, EntityToken> {
    scene
        .entities
        .iter_mut()
        .find(|entity| match (&ident, entity) {
            (EntityIdent::Apple(_), Entity::Apple(_)) => true,
            (EntityIdent::Book(_), Entity::Book(_)) => true,
            _ => false,
        })
        .ok_or(match ident {
            EntityIdent::NullEntity(token) => token,
            EntityIdent::Apple(token) => token,
            EntityIdent::Book(token) => token,
        })
}

fn do_instruction(scene: &mut Scene, instruction: Instruction) -> String {
    match instruction {
        Instruction::Exit => panic!("Can't do exit instruction on scene"),
        Instruction::Look => String::from("You look around and see an apple and a book."),
        Instruction::Describe(ident) => match find_entity(scene, ident) {
            Ok(entity) => match entity {
                Entity::Apple(apple) => apple.describe(),
                Entity::Book(book) => book.describe(),
            },
            Err(token) => format!("You can't find a {}", token),
        },
        Instruction::Consume(ident) => match find_entity(scene, ident) {
            Ok(entity) => {
                let result = match entity {
                    Entity::Apple(apple) => apple.consume(),
                    Entity::Book(_) => Err("It's not food."),
                };
                match result {
                    Ok(response) => response,
                    Err(error) => format!("{} You decide not to eat it.", error),
                }
            }
            Err(token) => format!("You can't find a {}.", token),
        },
        Instruction::Read(ident) => match find_entity(scene, ident) {
            Ok(entity) => {
                let result = match entity {
                    Entity::Book(book) => book.read(),
                    Entity::Apple(_) => Err("There's nothing to read."),
                };
                match result {
                    Ok(response) => response,
                    Err(error) => format!("{} You leave it alone.", error),
                }
            }
            Err(token) => format!("You can't find a {}.", token),
        },
    }
}

fn main() {
    let mut scene = game::Scene::new(vec![
        Entity::Apple(game::Apple::new()),
        Entity::Book(game::Book::new(
            String::from("The Lusty Argonian Maid"),
            String::from("Crassius Curio"),
            String::from("[contents here]"),
        )),
    ]);

    loop {
        let mut instruction = String::new();

        if let Err(error) = io::stdin().read_line(&mut instruction) {
            dbg!(error);
        } else {
            match parser::Instruction::try_from(instruction) {
                Err(error) => println!("{}\n", error),
                Ok(instruction) => {
                    if instruction == parser::Instruction::Exit {
                        std::process::exit(0);
                    }
                    println!("{}\n", do_instruction(&mut scene, instruction));
                }
            }
        }
    }
}
