use std::io;

use colored::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AbilityAttrs {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Ability {
    ability: AbilityAttrs,
}

#[derive(Debug, Deserialize)]
struct TypeAttrs {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Type {
    r#type: TypeAttrs,
}

#[derive(Debug, Deserialize)]
struct Pokemon {
    name: String,
    abilities: Vec<Ability>,
    height: f64,
    types: Vec<Type>,
    weight: f64,
}

impl Pokemon {
    fn print(&self) {
        let mut types: Vec<String> = vec![];
        let mut abilities: Vec<String> = vec![];

        for i in self.types.iter() {
            types.push(i.r#type.name.clone());
        }

        for i in self.abilities.iter() {
            abilities.push(i.ability.name.clone());
        }

        let text = format!(
            "Pokemon: {}
            > Weight: {},
            > Height: {},
            > Type: {},
            > Abilities: {}",
            self.name,
            self.weight,
            self.height,
            types.join(", "),
            abilities.join(", ")
        );

        println!("{}", text)
    }
}

fn main() {
    println!("{}", "Welcome to pokemon search!".bright_blue());
    loop {
        let mut pkm_name = String::new();

        println!("{}", "Enter pokemon name:".bright_yellow());
        match io::stdin().read_line(&mut pkm_name) {
            Ok(_) => {
                let pkm_name = pkm_name.trim();
                match get_pokemon(pkm_name) {
                    Ok(p) => p.print(),
                    Err(e) => eprintln!("Error getting pokemon: {}", e.to_string().bright_red()),
                }
            }
            Err(e) => eprintln!("Error parsing pokemon name: {}", e.to_string().bright_red()),
        }

        let mut try_again = String::new();

        println!("Do you want to search another pokemon? (y/n):");
        match io::stdin().read_line(&mut try_again) {
            Ok(_) => {
                let try_again = try_again.trim();

                if try_again == "yes" || try_again == "y" {
                    continue;
                } else {
                    println!("Bye!");
                    break;
                }
            }
            Err(e) => {
                println!("Error: {}, Bye!", e.to_string());
                break;
            }
        }
    }
}

fn get_pokemon(name: &str) -> Result<Pokemon, reqwest::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);

    let res = reqwest::blocking::get(url)?;
    let data = res.json::<Pokemon>()?;

    Ok(data)
}
