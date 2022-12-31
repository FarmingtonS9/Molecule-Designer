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

    let element = &element_list[17]; //Argon
    let mut n_matrix: DMatrix<i32> =
        DMatrix::from_element(element.period as usize, element.period as usize, 0);

    let mut electron_configuration_vector: Vec<i32> = Vec::new();
    let mut remaining_electrons = element.num_of_electrons;

    println!("Element: {}, Period: {}", element.element, element.period);
    println!("N-matrix {:?}", n_matrix);
    let mut row: usize;
    let mut column: usize;
    //Remember, n is the principal quantum number, which is equalavent to the element's period
    for n in 0..element.period as usize {
        println!("n = {}", n);
        for j in 0..(n + 1) {
            //sets the position based off where in the loop it is.
            //The combined values of row and column equal to n
            //Row increments with j; column decrements from n
            row = j;
            column = n - row;
            let position = (row, column);
            //Checks if position is in the lower triangular matrix
            if row >= column {
                //Calculates the number of electrons based on l (column)
                let electron_num = 2 * ((column as i32 * 2) + 1);
                //Checks if remaining number of electrons is less than the calculated value
                //If so, the calculated value is replaced with remaining number of electrons
                //Else, position keeps the calculated value and subtracts that vaule from remaining number of electrons
                if remaining_electrons < electron_num {
                    n_matrix[(position)] = remaining_electrons;
                } else {
                    n_matrix[(position)] = electron_num;
                    remaining_electrons -= electron_num;
                }
            }
            //Adds the value at position to the electron configuration if value does not equal zero
            if n_matrix[(position)] != 0 {
                electron_configuration_vector.push(n_matrix[(position)])
            }
            println!(
                "row number = {}, column number = {}, remaining electrons = {} position: {:?}, value at position: {}",
                row,
                column,
                remaining_electrons,
                position,
                n_matrix[(position)]
            )
        }
        if n == (element.period - 1) as usize && remaining_electrons > 0 {
            electron_configuration_vector.push(remaining_electrons)
        }
    }

    println!("N-Matrix: {:?}", n_matrix);
    println!(
        "Electron configuration: {:?}",
        electron_configuration_vector
    );

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
