use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;

mod chemistry;
use crate::chemistry::*;


const GENERATE_FILE_FOR_CLOUD: &str =
    r"G:\Cloud\Google Drive\Learning\Reading\Manufacturing\wind_turbine.txt";


const PROCESS_PLACEHOLDER: &str = "Find name of process";
const MANUFACTURED: &str = "Manufactured";
const MINED: &str = "Mined";

const OCTET_NUMBER: u32 = 8;
const ENERGY_SUBLEVEL: [&str; 4] = ["s", "p", "d", "f"];
const PRINCIPLE_ENERGY_LEVEL: [i32; 4] = [1, 2, 3, 4];

fn main() {
    println!("Hello, world!");
    println!("I am going to create a material and process tracker.");
    println!("Why? Because I want to keep track of materials and their associated processes");

    //wind_turbine_graph();

    let file_path = r"..\material-and-process-tracker\Periodic Table of Elements.csv";

    if let Ok(atom_collection) = read_csv_data(file_path) {
        let mut vector_of_elements: Vec<String> = Vec::new();
        let mut element_particles: Vec<Particles> = Vec::new();

        //Vec of Atom now works
        for atom in &atom_collection {
            println!("{}", atom)
        }

        let names_from_atom_collection = atom_collection.clone();
        for name in names_from_atom_collection {
            vector_of_elements.push(name.element)
        }
        //println!("{:?}", element_names);

        let number_of_particles_atom_collection = atom_collection.clone();
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
        //println!("{}", element_particles)

        let carbon = atom_from_atom_collection("Carbon", &vector_of_elements, &atom_collection);
        println!("{}", &carbon);

        let sodium = atom_from_atom_collection("Sodium", &vector_of_elements, &atom_collection);
        let chlorine = atom_from_atom_collection("Chlorine", &vector_of_elements, &atom_collection);
        println!("{}, {}", sodium, chlorine);

        let silicon = atom_from_atom_collection("Silicon", &vector_of_elements, &atom_collection);
        println!("{}", silicon.diff_from_full_valence());

        let aluminium =
            atom_from_atom_collection("Aluminium", &vector_of_elements, &atom_collection);
        let oxygen = atom_from_atom_collection("Oxygen", &vector_of_elements, &atom_collection);

        println!("{}", aluminium);
        println!("{}", oxygen);
        println!(
            "{}:{}",
            aluminium.calculate_valence_electrons(),
            oxygen.diff_from_full_valence()
        );
        molecule_designer(aluminium, oxygen);

        /*
        for n in PRINCIPLE_ENERGY_LEVEL {
            num_of_orbitals = i32::pow(n, 2);
            max_electrons = 2 * i32::pow(n, 2);
            println!("{}, {}", num_of_orbitals, max_electrons)
        }
        */

        //wind_turbine_graph()
    }
}

fn read_csv_data(file_path: &str) -> Result<Vec<Atom>, Box<dyn Error>> {
    let mut atom_collection: Vec<Atom> = Vec::new();
    let mut atom: Atom;

    let mut rdr = Reader::from_path(file_path).expect("Could not open csv file");

    for result in rdr.deserialize() {
        match result {
            Ok(atom) => atom_collection.push(atom),
            Err(err) => panic!(),
        };
    }
    Ok(atom_collection)
}

fn find_element_in_atom_collection(element_name: &str, vector_of_elements: &Vec<String>) -> usize {
    let element_name = element_name.to_string();
    let vector_of_elements = vector_of_elements;
    let index = vector_of_elements
        .iter()
        .position(|r| r.as_str() == element_name)
        .expect("Cannot find index");
    index
}

fn atom_from_atom_collection<'a>(
    element_name: &'a str,
    vector_of_elements: &'a Vec<String>,
    atom_collection: &'a Vec<Atom>,
) -> &'a Atom {
    let element_index = find_element_in_atom_collection(element_name, vector_of_elements);
    let atom = &atom_collection[element_index];
    atom
}

fn molecule_designer(first_element: &Atom, second_element: &Atom) {
    let first_diff_from_full_valence = first_element.diff_from_full_valence();
    let second_diff_from_full_valence = second_element.calculate_valence_electrons();

    let num_of_first_elements = second_diff_from_full_valence;
    let num_of_second_elements = first_diff_from_full_valence;

    println!(
        "Molecule designed: {}{}{}{}",
        first_element.symbol, num_of_first_elements, second_element.symbol, num_of_second_elements
    )
}
