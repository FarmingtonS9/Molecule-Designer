#![allow(unused)]

use chemistry::elements::*;
use csv::Reader;
use std::{error::Error, io};

mod chemistry;
use crate::chemistry::elements;

fn main() -> core::result::Result<(), io::Error> {
    println!("Hello, world!");
    println!("I am going to create a Molecule Designer.");
    println!("Why? Because I want to create molecules using the ideas of chemistry and physics. I want to do this to explore the fields of chemistry and physics. Eventually, I would love to keep track of materials and their associated processes.");

    let file_path = r"Periodic Table of Elements.csv";
    let element_list = read_csv(file_path).expect("Could not create element_list.");

    let element = &element_list[0];
    println!(
        "Element: {}; Period: {}, Electron configuration: {:?}",
        element.element,
        element.period,
        element.electron_configuration()
    );

    let element = &element_list[1];
    println!(
        "Element: {}; Period: {},  Electron configuration: {:?}",
        element.element,
        element.period,
        element.electron_configuration()
    );

    let element = &element_list[4];
    println!(
        "Element: {}; Period: {}, Electron configuration: {:?}",
        element.element,
        element.period,
        element.electron_configuration()
    );

    let element = &element_list[6];
    println!(
        "Element: {}; Period: {}, Electron configuration: {:?}",
        element.element,
        element.period,
        element.electron_configuration()
    );

    let element = &element_list[9];
    println!(
        "Element: {}; Period: {}, Electron configuration: {:?}",
        element.element,
        element.period,
        element.electron_configuration()
    );

    let element = &element_list[16];
    println!(
        "Element: {}; Period: {}, Electron configuration: {:?}",
        element.element,
        element.period,
        element.electron_configuration()
    );

    let element = &element_list[25];
    println!(
        "Element: {}; Period: {}, Electron configuration: {:?}",
        element.element,
        element.period,
        element.electron_configuration()
    );

    let element = &element_list[36];
    println!(
        "Element: {}; Period: {}, Electron configuration: {:?}",
        element.element,
        element.period,
        element.electron_configuration()
    );

    let element = &element_list[117];
    println!(
        "Element: {}; Period: {}, Electron configuration: {:?}",
        element.element,
        element.period,
        element.electron_configuration()
    );

    element.electron_vector();

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
    fn boron() {
        let element_list = get_element();
        let element = &element_list[4];
        assert_eq!("Boron".to_string(), element.element);
        assert_eq!(vec![2, 2, 1], element.electron_configuration())
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
