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

    let array = [1, 2, 3, 4, 5, 6];

    let mut matrix = DMatrix::from_iterator(3, 2, array.iter().cloned());
    println!("Matrix: {:?}", matrix);
    matrix[(0, 1)] = -13;
    println!("Matrix: {:?}", matrix);

    let element = &element_list[53]; //Xenon
    let mut n_matrix: DMatrix<i32> =
        DMatrix::from_element(element.period as usize, element.period as usize, 0);

    let mut electron_configuration_vector: Vec<i32> = Vec::new();
    let mut remaining_electrons = element.num_of_electrons;

    println!("Element: {}, Period: {}", element.element, element.period);
    println!("N-matrix {:?}", n_matrix);
    let mut row: usize = 0;
    let mut column: usize = 0;
    let mut position: (usize, usize) = (0, 0);
    //Remember, n is the principal quantum number, which is equalavent to the element's period
    for n in 0..(element.period + 1) as usize {
        column = n / 2;
        row = n - column;
        position = (row, column);
        n_matrix[(position)] = 2 * ((column as i32 * 2) + 1);
        println!(
            "row = {}, column = {}, n = {}, position = {:?}, val at pos = {:?}",
            row,
            column,
            n,
            position,
            n_matrix[(position)]
        );
        let range = column + 1;
        for j in 0..range {
            column -= j;
            row += j;
            position = (row, column);
            n_matrix[(position)] = 2 * ((column as i32 * 2) + 1);
            println!(
                "row = {}, column = {}, n = {}, j = {}, range = {}, position = {:?}, val at postion = {}",
                row,
                column,
                n,
                j,
                range,
                position,
                n_matrix[(position)]
            );
        }
    }

    println!(
        "N-Matrix: {:?}, N-Matrix value = {}",
        n_matrix,
        n_matrix.sum()
    );
    println!(
        "Electron configuration: {:?}",
        electron_configuration_vector
    );

    let value = 5 as usize;
    let calc_value = value / 2;
    println!("value = {}, calc_value = {}", value, calc_value);

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
