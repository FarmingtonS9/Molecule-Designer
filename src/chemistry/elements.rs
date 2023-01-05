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
    //Cheat/precalculated subshell numbers.
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

    //Algorithm for calculating electron configuration
    //Doesn't take into account edge cases; just calculates config based on number of electrons and the atomic period
    fn electron_configuration_v2(&self) -> Vec<i32> {
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
    LigandField,
}
