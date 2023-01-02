#![allow(unused)]

use na::{DMatrix, Dynamic, Matrix1, VecStorage};
//Imports
use nalgebra as na;
use serde::Deserialize;
use std::{fmt::Display, vec};

use crate::chemistry::models::*;

//Constants
//Private
const SHELL: [&str; 7] = ["K", "L", "M", "N", "O", "P", "Q"];
const PRINCIPAL_QUANTUM_NUM_ARRAY: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
const AZIMUTHAL_QUANTUM_NUM_ARRAY: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
const ORBITAL_SYMBOLS: [&str; 7] = ["s", "p", "d", "f", "g", "h", "i"];
const ORBITAL_NUM: [i32; 4] = [1, 3, 5, 7];

//Cheeky hardcode of subshell values in order hehe
const HC_SUBSHELL: [&str; 19] = [
    "1s", "2s", "2p", "3s", "3p", "4s", "3d", "4p", "5s", "4d", "5p", "6s", "4f", "5d", "6p", "7s",
    "5f", "6d", "7p",
];
//Cheat subshell numbers; used until it can be calculated algorithmically
const CHEAT_SUBSHELL_NUMS: [i32; 19] = [
    2, 2, 6, 2, 6, 2, 10, 6, 2, 10, 6, 2, 14, 10, 6, 2, 14, 10, 6,
];

//Public
//Structs (data)
//Find the minimum amount of information needed for other information to be derived from.
//E.g Atomic Number (Z) can be derived directly from the number of protons.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct Element {
    #[serde(rename = "Element")]
    pub element: String,
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "AtomicNumber")]
    pub atomic_num: i32,
    #[serde(rename = "AtomicMass")]
    pub atomic_mass: f32,
    #[serde(rename = "NumberofProtons")]
    pub num_of_protons: i32,
    #[serde(rename = "NumberofNeutrons")]
    pub num_of_neutrons: i32,
    #[serde(rename = "NumberofElectrons")]
    pub num_of_electrons: i32,
    #[serde(rename = "Period")]
    pub period: i32,
}

//RawElement handles the raw data input from a file, i.e .csv, and enables a "clean" Element struct
//Try to use RawElement to build an Element struct. I.e, deriving values for fields from raw data.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
struct RawElement {
    #[serde(rename = "Element")]
    pub element: String,
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "AtomicMass")]
    pub atomic_mass: f32,
    #[serde(rename = "NumberofProtons")]
    pub num_of_protons: i32,
    #[serde(rename = "NumberofNeutrons")]
    pub num_of_neutrons: i32,
    #[serde(rename = "NumberofElectrons")]
    pub num_of_electrons: i32,
    #[serde(rename = "Period")]
    pub period: i32,
}

//Temporary, move to particles file
#[derive(Debug)]
pub struct Electron {
    spin: bool,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Element: {}, Symbol: {}, Atomic Number: {}, Protons: {}, Neutrons: {}, Electrons: {}",
            self.element,
            self.symbol,
            self.atomic_num,
            self.num_of_protons,
            self.num_of_neutrons,
            self.num_of_electrons,
        )
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            element: String::from("Default Atom"),
            symbol: String::from("De"),
            atomic_num: 1,
            atomic_mass: 1.,
            num_of_protons: 1,
            num_of_neutrons: 1,
            num_of_electrons: 1,
            period: 1,
        }
    }
}

//Public associated functions
impl Element {
    //Still to decide, whether to create molecules through Atom or Molecule structs, or do something funky with traits
    //Electron configuration is pre-calculated
    pub fn electron_configuration(&self) -> Vec<i32> {
        self.electron_configuration_v2()
    }

    //Second attempt
    //Uses properties of matrices to calculate values in the lower triangular matrix
    //I.e Using division and iteration to keep track of position
    pub fn electron_configuration_v2(&self) -> Vec<i32> {
        let element = self;
        let mut n_matrix: DMatrix<i32> =
            DMatrix::from_element(element.period as usize, element.period as usize, 0);

        let mut electron_configuration_vector: Vec<i32> = Vec::new();
        let mut remaining_electrons = element.num_of_electrons;

        let mut row: usize = 0;
        let mut column: usize = 0;
        let mut position: (usize, usize) = (0, 0);
        //Remember, n is the principal quantum number, which is equalavent to the element's period
        for n in 0..(element.period + 1) as usize {
            column = n / 2;
            row = n - column;
            position = (row, column);

            for j in 0..column + 1 {
                //Sets the position within the matrix
                let column = column - j;
                let row = row + j;
                position = (row, column);

                //A check to ensure row value doesn't try to access matrix out of bounds
                //And checks if remaining electrons are zero; issue with loop continuing to calculate even when there were no electrons
                //Skips this iteration of for loop
                if row == element.period as usize || remaining_electrons == 0 {
                    continue;
                }

                //Logic
                //Calculates the remaining electrons left in this subshell
                n_matrix[(position)] = 2 * ((column as i32 * 2) + 1);
                //If there are more remaining electrons than the calculated value
                if remaining_electrons > n_matrix[(position)] {
                    //Add the calculated value to the electron configuration vector
                    electron_configuration_vector.push(n_matrix[(position)]);
                    //Subtract calculated value from the remaining electrons
                    remaining_electrons -= n_matrix[(position)]
                } else {
                    //Set the element in the matrix to the remaining electrons
                    n_matrix[(position)] = remaining_electrons;
                    //Take the remaining electrons away
                    remaining_electrons -= remaining_electrons;
                    //Set the last element of the electron vector to the remaining electrons
                    electron_configuration_vector.push(n_matrix[(position)]);
                };
            }
        }
        electron_configuration_vector
    }

    pub fn test_data(&self) {
        let sum: i32 = self.electron_configuration().iter().sum();
        println!(
            "Element: {}; Atomic Number: {}, Period: {}, Electron configuration: {:?} -> Sum: {}",
            self.element,
            self.atomic_num,
            self.period,
            self.electron_configuration(),
            sum
        );
    }
}

//Traits implemented for Element
//Lewis Structures
impl LewisStructures for Element {}

//Private associated functions
impl Element {
    //NEW, UPDATED, ROBUST algorithms here.
    //Madelung Rule
    //n + l = subshell energy, determines order with lower n values preferred first in case of repeated values
    //May not need this function
    fn madelung_num(principal_num: i32, azimuthal_num: i32) -> i32 {
        principal_num + azimuthal_num
    }

    //Determine shell
    fn det_shell(&self) -> (i32, i32) {
        //Element's number of shells
        let mut element_no_of_electron = self.num_of_electrons;
        let mut principal_quantum_num = 1;
        let mut remaining_electrons: i32 = 0;
        //Figure out the shell, iterating(?) until element's electrons are exhausted
        //Calculate first element in PRINCIPAL_QUANTUM_NUM array using 2*n^2
        //Subtract this value from Element's electrons
        //Repeat until Element's electrons is less than calculated value
        for num in PRINCIPAL_QUANTUM_NUM_ARRAY.iter() {
            let mut temp_val = 2 * (num.pow(2));
            if element_no_of_electron > temp_val {
                element_no_of_electron -= temp_val;
                principal_quantum_num += 1;
            } else {
                remaining_electrons = element_no_of_electron;
                break;
            }
        }
        //Output principal quantum number and remaining electrons
        (principal_quantum_num, remaining_electrons)
    }

    //Determine subshell, based on det_shell()
    fn det_subshell(&self) {
        //Values passed from det_shell() function
        let shell_tuple = self.det_shell();
        //Resulting values
        let principal_quantum_num = shell_tuple.0;
        let mut element_no_of_electrons = self.num_of_electrons;

        //Create slice of principal quantum numbers
        let principal_quantum_num_slice =
            &PRINCIPAL_QUANTUM_NUM_ARRAY[..principal_quantum_num as usize];

        //Iterate over each element in the principal quantum number (n) slice
        //Each iteration over n, then iterate over the azimuthal quantum number slice
        //To calculate the corresponding maximum number of electrons per subshell
        //
        //Equation: 2 * ((2 * l) + 1)
        //Vector to hold max no. of electrons per subshell
        let mut madelung_num_vector: Vec<usize> = Vec::new();
        let mut azimuthal_num_vector: Vec<i32> = Vec::new();

        for n in principal_quantum_num_slice.iter() {
            //n determines where we are in the principal slice
            //
            //At each value of n, enter l and generate max no. of electrons
            //Take a slice of the azimuthal quantum number (l) array based on the principal quantum number (n)
            let azimthual_quantum_num_slice = &AZIMUTHAL_QUANTUM_NUM_ARRAY[..(*n as usize)];

            for l in azimthual_quantum_num_slice.iter() {
                //Add azithumal number to vector
                azimuthal_num_vector.push(*l);

                //Madelung's number
                let madelungs_num = (*n + azimthual_quantum_num_slice[*l as usize]) as usize;
                madelung_num_vector.push(madelungs_num);

                //
                let electrons_in_subshell =
                    2 * ((2 * azimthual_quantum_num_slice[*l as usize]) + 1);

                if element_no_of_electrons > electrons_in_subshell {
                    element_no_of_electrons -= electrons_in_subshell
                }
            }
        }

        let electron_config = self.precalculated_subshells();

        //2nd attempt;
        let principal_num = self.period;
        let mut azimuthal_num = 0;
        let mut num_subshell_electrons = 0;
        let principal_num_array = &PRINCIPAL_QUANTUM_NUM_ARRAY[..principal_num as usize];

        println!(
            "Principal quantum number: {}, electrons in outer shell: {}, azimuthal quantum number: {:?}, Madelung's numbers: {:?}, electron configuration: {:?}",
            principal_quantum_num,
            electron_config.last().unwrap(),
            azimuthal_num_vector,
            madelung_num_vector,
            electron_config
        )
    }

    //Cheat/precalculated subshell numbers.
    //Will need to come back and try to create this algorithm properly
    fn precalculated_subshells(&self) -> Vec<i32> {
        let mut num_electrons = self.num_of_electrons;
        let mut remaining_electrons = 0;
        let mut subshell_slice: Vec<i32> = Vec::new();

        for num in CHEAT_SUBSHELL_NUMS.iter() {
            if num_electrons > *num {
                num_electrons -= *num;
                subshell_slice.push(*num)
            } else {
                remaining_electrons = num_electrons;
                subshell_slice.push(remaining_electrons);
                break;
            }
        }
        subshell_slice
    }

    //Constructing matrix
    //Size of matrix N = period * period
    //Selecting element position in matrix is N[(r - 1, c - 1)], where r/c are max values of n (the Element's period)
    //Retrieve a slice; set Madelung's number to (n + l) - 1, retrieve coordinates where equal to Madelung's number
    //
}

#[derive(Debug)]
pub struct Molecule {
    pub name: String,
    pub formula: String,
}

impl Molecule {
    pub fn new(name: String, formula: String) -> Self {
        Molecule { name, formula }
    }

    //Still to decide, whether to create molecules through Atom (currently a trait) or Molecule structs
}

impl Display for Molecule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{}", self.name, self.formula)
    }
}

#[derive(Debug)]
pub struct Particles {
    pub protons: i32,
    pub neutrons: i32,
    pub electrons: i32,
}

impl Display for Particles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Protons: {}, Neutrons: {}, Electrons: {}",
            self.protons, self.neutrons, self.electrons
        )
    }
}

//Enums
//Phases
//To implement when enthalpy is implemented
pub enum Phase {
    Solid,
    Liquid,
    Gas,
    Plasma,
}

//Capturing the various diagrams representing the electron configuration of an atom/molecule
//Currently unused
pub enum ElectronConfig {
    LewisDotSymbol,
    CouperKekuleDiagrams,
    OrbitalDiagram,
    SPDFNotation,
    SPDFNotationExpanded,
    NobleGasNotation,
}
