#![allow(unused)]

use csv::Reader;
use na::*;
use nalgebra as na;
use std::{error::Error, io};

mod chemistry;
pub mod physics;
use chemistry::elements::*;

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    println!("I am going to create a Molecule Designer.");
    println!("Why? Because I want to create molecules using the ideas of chemistry and physics. I want to do this to explore the fields of chemistry and physics. Eventually, I would love to keep track of materials and their associated processes.");

    let file_path = r"Periodic Table of Elements.csv";
    let element_list = read_csv(file_path).expect("Could not create element_list.");

    let element = &element_list[0];
    element.test_data();

    let element = &element_list[1];
    element.test_data();

    let element = &element_list[4];
    element.test_data();

    let element = &element_list[6];
    element.test_data();

    let element = &element_list[9];
    element.test_data();

    let element = &element_list[16];
    element.test_data();

    let element = &element_list[25];
    element.test_data();

    let element = &element_list[36];
    element.test_data();

    let element = &element_list[117];
    element.test_data();

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

#[cfg(test)]
mod element_tests {
    use crate::{chemistry::elements::*, read_csv};
    use std::vec;

    const FILE_PATH: &str = r"Periodic Table of Elements.csv";

    fn get_element() -> Vec<Element> {
        let element_list = read_csv(FILE_PATH).expect("Could not create element_list.");
        element_list
    }

    #[test]
    fn hydrogen() {
        let element_list = get_element();
        let element = &element_list[0];
        assert_eq!("Hydrogen".to_string(), element.element);
        assert_eq!(vec![1], element.electron_configuration())
    }
    #[test]
    fn helium() {
        let element_list = get_element();
        let element = &element_list[1];
        assert_eq!("Helium".to_string(), element.element);
        assert_eq!(vec![2], element.electron_configuration())
    }
    #[test]
    fn lithium() {
        let element_list = get_element();
        let element = &element_list[2];
        assert_eq!("Lithium".to_string(), element.element);
        assert_eq!(vec![2, 1], element.electron_configuration())
    }
    #[test]
    fn boron() {
        let element_list = get_element();
        let element = &element_list[4];
        assert_eq!("Boron".to_string(), element.element);
        assert_eq!(vec![2, 2, 1], element.electron_configuration())
    }
    #[test]
    fn carbon() {
        let element_list = get_element();
        let element = &element_list[5];
        assert_eq!("Carbon".to_string(), element.element);
        assert_eq!(vec![2, 2, 2], element.electron_configuration())
    }
    #[test]
    fn nitrogen() {
        let element_list = get_element();
        let element = &element_list[6];
        assert_eq!("Nitrogen".to_string(), element.element);
        assert_eq!(vec![2, 2, 3], element.electron_configuration())
    }
    #[test]
    fn neon() {
        let element_list = get_element();
        let element = &element_list[9];
        assert_eq!("Neon".to_string(), element.element);
        assert_eq!(vec![2, 2, 6], element.electron_configuration())
    }
    #[test]
    fn sodium() {
        let element_list = get_element();
        let element = &element_list[10];
        assert_eq!("Sodium".to_string(), element.element);
        assert_eq!(vec![2, 2, 6, 1], element.electron_configuration())
    }
    #[test]
    fn magnesium() {
        let element_list = get_element();
        let element = &element_list[11];
        assert_eq!("Magnesium".to_string(), element.element);
        assert_eq!(vec![2, 2, 6, 2], element.electron_configuration())
    }
    #[test]
    fn potassium() {
        let element_list = get_element();
        let element = &element_list[18];
        assert_eq!("Potassium".to_string(), element.element);
        assert_eq!(vec![2, 2, 6, 2, 6, 1], element.electron_configuration())
    }

    #[test]
    fn iron() {
        let element_list = get_element();
        let element = &element_list[25];
        assert_eq!("Iron".to_string(), element.element);
        assert_eq!(vec![2, 2, 6, 2, 6, 2, 6], element.electron_configuration())
    }
    #[test]
    #[should_panic]
    //Palladium is an interesting example of shortcomings of Aufbau
    //Electron configuration is experimentally known as: [1s2, 2s2, 2p6, 3s2, 3p6, 4s2, 3d10, 4p6, 4d10] (correct)
    //Electron configuration is calculated here as: [1s2, 2s2, 2p6, 3s2, 3p6, 4s2, 3d10, 4p6, 5s2, 4d8] (incorrect)
    //Instead of filling the s subshell, the d subshell is able to be filled completely
    fn palladium() {
        let element_list = get_element();
        let element = &element_list[45];
        assert_eq!("Palladium".to_string(), element.element);
        assert_eq!(
            vec![2, 2, 6, 2, 6, 2, 10, 6, 10],
            element.electron_configuration()
        )
    }

    #[test]
    fn francium() {
        let element_list = get_element();
        let element = &element_list[86];
        assert_eq!("Francium".to_string(), element.element);
        assert_eq!(
            vec![2, 2, 6, 2, 6, 2, 10, 6, 2, 10, 6, 2, 14, 10, 6, 1],
            element.electron_configuration()
        )
    }
    #[test]
    #[should_panic] //Electron configuration is incorrectly calculated based on Aufbau principle
    fn uranium() {
        let element_list = get_element();
        let element = &element_list[91];
        assert_eq!("Uranium".to_string(), element.element);
        assert_eq!(
            vec![2, 2, 6, 2, 6, 2, 10, 6, 2, 10, 6, 2, 14, 10, 6, 2, 3, 1],
            element.electron_configuration()
        );
    }
    #[test]
    fn oganesson() {
        let element_list = get_element();
        let element = &element_list[117];
        assert_eq!("Oganesson".to_string(), element.element);
        assert_eq!(
            vec![2, 2, 6, 2, 6, 2, 10, 6, 2, 10, 6, 2, 14, 10, 6, 2, 14, 10, 6],
            element.electron_configuration()
        )
    }
    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let element_list = get_element();
        let element = &element_list[119];
    }
}

//Finds index of element
fn find_index_of_element(element_name: &str, vector_of_elements: &[String]) -> usize {
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
    vector_of_elements: &'a [String],
    atom_list: &'a [Element],
) -> &'a Element {
    let element_index = find_index_of_element(element_name, vector_of_elements);
    &atom_list[element_index] as _
}
