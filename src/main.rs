#![allow(unused)]

use csv::Reader;
use std::{error::Error, io};

mod chemistry;
use crate::chemistry::*;

const OCTET_NUMBER: u32 = 8;
const PRINCIPLE_ENERGY_LEVEL: [i32; 4] = [1, 2, 3, 4];
const NOBLE_GAS_ELEMENT_NUMBER: [i32; 7] = [2, 10, 18, 36, 54, 86, 118];

fn main() -> core::result::Result<(), io::Error> {
    println!("Hello, world!");
    println!("I am going to create a Molecule Designer.");
    println!("Why? Because I want to create molecules using the ideas of chemistry and physics. I want to do this to explore the fields of chemistry and physics. Eventually, I would love to keep track of materials and their associated processes.");

    let file_path = r"Periodic Table of Elements.csv";
    let element_list = read_csv(file_path).expect("Could not create element_list.");

    /* TODO. Building a tui/gui interface
        match tui_entry(atom_list) {
            Ok(()) => {},
            Err(err) => println!("{:?}", err)
        }
    */
    let input = 1;

    let element = &element_list[input - 1];
    let valence_electrons = element.valence_electrons();
    println!(
        "Element: {}, Valence electrons: {}",
        &element.element, valence_electrons
    );

    let aluminium = &element_list[12];
    let oxygen = &element_list[7];

    println!(
        "Element 1: {}, Element 2: {}\n",
        &aluminium.element, &oxygen.element
    );

    for element in element_list.iter() {
        println!("{}", element);
    }

    let alum_oxide = Element::create_molecule(aluminium, oxygen);
    println!("{}", alum_oxide);

    let alum_oxide2 = Molecule::create_molecule(aluminium, oxygen);
    println!("{}", alum_oxide2);

    let hydrogen = &element_list[0];
    let diatomic_hydrogen = Molecule::create_molecule(hydrogen, hydrogen);
    println!("{}", diatomic_hydrogen);

    let helium = &element_list[1];
    println!("{}", helium.lewis_dot_symbol());

    let element = &element_list[17];
    print!("{} code: ", &element.element);
    element.get_electron_quantum_nums();

    let element = &element_list[0];
    println!("Element: {}", element.element);
    element.det_subshell();

    let element = &element_list[1];
    println!("Element: {}", element.element);
    element.det_subshell();

    let element = &element_list[4];
    println!("Element: {}", element.element);
    element.det_subshell();

    let element = &element_list[6];
    println!("Element: {}", element.element);
    element.det_subshell();

    let element = &element_list[9];
    println!("Element: {}", element.element);
    element.det_subshell();

    let element = &element_list[16];
    println!(
        "Element: {}, Config: {:?}",
        element.element,
        element.precalculated_subshells()
    );

    let element = &element_list[36];
    println!(
        "Element: {}, Config: {:?}",
        element.element,
        element.precalculated_subshells()
    );

    let electron_config = element.precalculated_subshells();
    println!("Electron Configuration: {:?}", electron_config);

    Ok(())
}

fn read_csv(file_path: &str) -> Result<Vec<Element>, Box<dyn Error>> {
    let mut elements: Vec<Element> = Vec::new();
    let mut rdr = Reader::from_path(file_path).expect("Could not open csv file.");

    for result in rdr.deserialize() {
        match result {
            Ok(atom) => elements.push(atom),
            Err(err) => println!("{}", err),
        };
    }
    Ok(elements)
}

//Finds index of element
fn find_index_of_element(element_name: &str, vector_of_elements: &Vec<String>) -> usize {
    let element_name = element_name.to_string();
    let vector_of_elements = vector_of_elements;
    let index = vector_of_elements
        .iter()
        .position(|r| r.as_str() == element_name)
        .expect("Cannot find element name");
    index
}

fn find_atom_using_index<'a>(
    element_name: &'a str,
    vector_of_elements: &'a Vec<String>,
    atom_list: &'a Vec<Element>,
) -> &'a Element {
    let element_index = find_index_of_element(element_name, vector_of_elements);
    let atom = &atom_list[element_index];
    atom
}
