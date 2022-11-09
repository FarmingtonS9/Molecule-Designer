use csv::Reader;
use std::{error::Error, io};

mod chemistry;
mod tui;
use crate::chemistry::*;
use crate::tui::*;

const OCTET_NUMBER: u32 = 8;
const ENERGY_SUBLEVEL: [&str; 4] = ["s", "p", "d", "f"];
const PRINCIPLE_ENERGY_LEVEL: [i32; 4] = [1, 2, 3, 4];
const NOBLE_GAS_ELEMENT_NUMBER: [i32; 7] = [2, 10, 18, 36, 54, 86, 118];

fn main() -> core::result::Result<(), io::Error> {
    println!("Hello, world!");
    println!("I am going to create a Molecule Designer.");
    println!("Why? Because I want to create molecules using the ideas of chemistry and physics. I want to do this to explore the fields of chemistry and physics. Eventually, I would love to keep track of materials and their associated processes.");

    let file_path = r"..\material-and-process-tracker\Periodic Table of Elements.csv";

    let atom_list = read_csv(file_path).expect("Could not create atom_list.");
    for atom in atom_list {
        println!("{}", atom)
    }

    tui_setup();

    Ok(())
}

fn read_csv(file_path: &str) -> Result<Vec<Atom>, Box<dyn Error>> {
    let mut atoms: Vec<Atom> = Vec::new();
    let mut rdr = Reader::from_path(file_path).expect("Could not open csv file.");

    for result in rdr.deserialize() {
        match result {
            Ok(atom) => atoms.push(atom),
            Err(err) => println!("{}", err),
        };
    }
    Ok(atoms)
}

fn find_element_in_atom_collection(element_name: &str, vector_of_elements: &Vec<String>) -> usize {
    let element_name = element_name.to_string();
    let vector_of_elements = vector_of_elements;
    let index = vector_of_elements
        .iter()
        .position(|r| r.as_str() == element_name)
        .expect("Cannot find element name");
    index
}

fn atom_from_atom_collection<'a>(
    element_name: &'a str,
    vector_of_elements: &'a Vec<String>,
    atom_list: &'a Vec<Atom>,
) -> &'a Atom {
    let element_index = find_element_in_atom_collection(element_name, vector_of_elements);
    let atom = &atom_list[element_index];
    atom
}
