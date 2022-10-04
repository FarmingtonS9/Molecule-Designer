

use csv::Reader;
use std::{error::Error, io};

mod chemistry;
use crate::chemistry::*;

const OCTET_NUMBER: u32 = 8;
const ENERGY_SUBLEVEL: [&str; 4] = ["s", "p", "d", "f"];
const PRINCIPLE_ENERGY_LEVEL: [i32; 4] = [1, 2, 3, 4];

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    println!("I am going to create a material and process tracker.");
    println!("Why? Because I want to keep track of materials and their associated processes");

    let file_path = r"..\material-and-process-tracker\Periodic Table of Elements.csv";

    let mut vector_of_elements: Vec<String> = Vec::new();
    let mut element_particles: Vec<Particles> = Vec::new();

    if let Ok(atom_list) = read_csv(file_path) {
        //Vec of Atom now works
        for atom in &atom_list {
            println!("{}", atom)
        }

        let vector_atom = atom_list.clone();
        for name in vector_atom {
            vector_of_elements.push(name.element)
        }
        //println!("{:?}", element_names);

        let number_of_particles_atom_collection = atom_list.clone();
        for particles in number_of_particles_atom_collection {
            let protons = particles.num_of_protons;
            let neutrons = particles.num_of_neutrons;
            let electrons = particles.num_of_electrons;
            element_particles.push(Particles {
                protons,
                neutrons,
                electrons,
            })
        }

        let sodium = atom_from_atom_collection("Sodium", &vector_of_elements, &atom_list);
        assert_eq!(1, sodium.valence_electrons());
        let chlorine = atom_from_atom_collection("Chlorine", &vector_of_elements, &atom_list);
        assert_eq!(7, chlorine.valence_electrons());
    }



    Ok(())
}

fn read_csv(file_path: &str) -> Result<Vec<Atom>, Box<dyn Error>> {
    let mut atoms: Vec<Atom> = Vec::new();
    let mut atom: Atom;

    let mut rdr = Reader::from_path(file_path).expect("Could not open csv file");

    for result in rdr.deserialize() {
        match result {
            Ok(atom) => atoms.push(atom),
            Err(err) => panic!(),
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
