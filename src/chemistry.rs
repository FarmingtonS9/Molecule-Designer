#![allow(unused)]
/*To do
- Add enthalpy; determine its effects
    - First, add Standard Lab Conditions (SLC: Pressure: 1 atm, Temperature: 25(C) degrees/298.15(K))
    - Likely add the top physical quantities to the physics.rs file
*/

use na::{DMatrix, Dynamic, Matrix1, VecStorage};
//Imports
use nalgebra as na;
use serde::Deserialize;
use std::{fmt::Display, vec};

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
const CHEAT_SUBSHELL_NUMS: [i32; 19] = [
    2, 2, 6, 2, 6, 2, 10, 6, 2, 10, 6, 2, 14, 10, 6, 2, 14, 10, 6,
];

//Public

//Traits (shared behaviours/properties)
pub trait Atom {
    //Electron configuration of atom or molecule
    fn electronic_structure() {}
}

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

//To complete
impl Atom for Element {}

//Public associated functions
impl Element {
    //Used to determine if the element is electrically charged
    //I.e if an element binds to another atom, it may have lost/gained an electron, therefore it is now electrically charged (becomes an ion)
    pub fn calc_charge(&self) -> i32 {
        self.num_of_protons - self.num_of_electrons
    }
    //This outputs the number of valence electrons.
    //Works for now, may need to be updated in future.
    //Likely, will focus on balance of charges.
    pub fn valence_electrons(&self) -> i32 {
        let period = self.period;
        let electron = self.num_of_electrons;
        let max_valence: i32;
        let valence: i32;

        match period {
            1 => {
                max_valence = 2;
                valence = max_valence - electron
            }
            2 => valence = { 10 - electron },
            3 => valence = { 18 - electron },
            _ => todo!(),
        }
        //If number of electrons in outer shell is greater than 4, the number of valence electrons "steps" down.
        if valence > 4 {
            return valence - (valence - 4) * 2;
        }
        valence
    }

    //Still to decide, whether to create molecules through Atom or Molecule structs, or do something funky with traits
    pub fn create_molecule(elem1: &Self, elem2: &Element) -> Molecule {
        let elem1_symbol = &elem1.symbol;
        let elem1_valence = &elem1.valence_electrons();

        let elem2_symbol = &elem2.symbol;
        let elem2_valence = &elem2.valence_electrons();

        let mut formula = String::new();

        match elem1_symbol {
            elem2_symbol => {
                formula = format!("{}_{}", &elem1_symbol, *elem1_valence + *elem2_valence)
            }
            _ => {
                formula = format!(
                    "{}_{}:{}_{}",
                    &elem1_symbol, &elem2_valence, &elem2_symbol, &elem1_valence
                )
            }
        }

        let molecule_name = format!("{}-{}\n", &elem1.element, &elem2.element);

        Molecule {
            name: molecule_name,
            formula: formula,
        }
    }

    pub fn oxidation_states(&self) {
        todo!()
    }

    pub fn get_electron_quantum_nums(&self) {
        let quantum_nums = self.determine_orbit();
        let shell = quantum_nums.0;
        let principal_quantum_num = quantum_nums.1;
        let magnetic_quantum_num = quantum_nums.3;

        let angular_momentum = quantum_nums.2;
        let angular_momentum_symb = angular_momentum.0;
        let angular_momentum_quantum_num = angular_momentum.1;

        println!(
            "{}{}{}{}{}",
            shell,
            principal_quantum_num,
            angular_momentum_symb,
            angular_momentum_quantum_num,
            magnetic_quantum_num
        );
    }
}

//Impl for different representations of electron configurations
impl Element {
    //Lewis dot diagram for element.
    //Make this associated function more robust
    pub fn lewis_dot_symbol(&self) -> String {
        let dot = ["", "•", "••", ":"];

        let diagram = format!(
            "{:>2}\n{}{}{}\n{:>2}",
            dot[0], dot[1], &self.symbol, dot[1], dot[0]
        );
        diagram
    }

    pub fn orbital_diagram() {}

    pub fn spdf_notation() {}

    pub fn noble_gas_notation() {}
}

//Private associated functions
impl Element {
    //Determine the shell of the element
    //May have to rethink how to structure these functions.
    //Incorporate the 4 quantum numbers, maybe??
    fn determine_shell(&self) -> (String, i32, i32) {
        let max_num_of_electrons: i32;
        let mut shell = String::new();
        let principal_quantum_num: i32;

        //Based on number of electrons, return the shell, principal quantum number and maximum number of electrons
        //In this shell.
        //Restructure this to work out shell using the number of electrons (instead of hardcoding values)
        //Curently, outputting incorrect values because of the if conditions.
        if self.num_of_electrons <= 2 {
            shell = SHELL[0].to_string();
            principal_quantum_num = PRINCIPAL_QUANTUM_NUM_ARRAY[0];
            max_num_of_electrons = 2 * principal_quantum_num.pow(2);

            return (shell, principal_quantum_num, max_num_of_electrons);
        } else if self.num_of_electrons <= 8 {
            shell = SHELL[1].to_string();
            principal_quantum_num = PRINCIPAL_QUANTUM_NUM_ARRAY[1];
            max_num_of_electrons = 2 * principal_quantum_num.pow(2);

            return (shell, principal_quantum_num, max_num_of_electrons);
        } else if self.num_of_electrons <= 18 {
            shell = SHELL[2].to_string();
            principal_quantum_num = PRINCIPAL_QUANTUM_NUM_ARRAY[2];
            max_num_of_electrons = 2 * principal_quantum_num.pow(2);

            return (shell, principal_quantum_num, max_num_of_electrons);
        } else {
            shell = SHELL[3].to_string();
            principal_quantum_num = PRINCIPAL_QUANTUM_NUM_ARRAY[3];
            max_num_of_electrons = 2 * principal_quantum_num.pow(2);

            return (shell, principal_quantum_num, max_num_of_electrons);
        }
    }

    //Determine angular momentum quantum number
    fn determine_subshell(&self) -> (String, i32, (String, i32)) {
        let shell = self.determine_shell();
        let angular_momentum_quantum_num = shell.1 - 1;
        let index = angular_momentum_quantum_num as usize;
        let angular_momentum_symbol = ORBITAL_SYMBOLS[index].to_string();

        return (
            shell.0,
            shell.1,
            (angular_momentum_symbol, angular_momentum_quantum_num),
        );
    }

    //Determine magnetic quantum number
    //Update algorithm to return appropriate magnetic numbers (i.e )
    fn determine_orbit(&self) -> (String, i32, (String, i32), i32) {
        let subshell = self.determine_subshell();
        let angular_momentum_tuple = subshell.2;
        //Asks for principal quantum number, calculates magnetic number
        let magnetic_quantum_num = (2 * subshell.1) + 1;

        return (
            subshell.0,
            subshell.1,
            angular_momentum_tuple,
            magnetic_quantum_num,
        );
    }

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
                element_no_of_electron = element_no_of_electron - temp_val;
                principal_quantum_num = principal_quantum_num + 1;
            } else {
                remaining_electrons = element_no_of_electron;
                break;
            }
        }
        //Output principal quantum number and remaining electrons
        (principal_quantum_num, remaining_electrons)
    }

    //Determine subshell, based on det_shell()
    pub fn det_subshell(&self) {
        //Values passed from det_shell() function
        let shell_tuple = self.det_shell();
        //Resulting values
        let principal_quantum_num = shell_tuple.0;
        let electrons_outer_shell = shell_tuple.1;
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
        let mut max_number_electrons_subshell: Vec<i32> = Vec::new();
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
                    element_no_of_electrons = element_no_of_electrons - electrons_in_subshell
                }

                //Setting the position inside the max_electrons_subshell vector
                max_number_electrons_subshell.push(electrons_in_subshell);
            }
        }

        //Replacing the last value with remaining electrons value
        //Obtaining the actual number of electrons of the element closer to the electron configuration
        let mut element_subshell_config = max_number_electrons_subshell.clone();
        element_subshell_config.pop();
        let last_element = element_subshell_config.len();
        element_subshell_config.insert(last_element, element_no_of_electrons);

        //2nd attempt;

        println!(
            "Principal quantum number: {}, electrons in outer shell: {}, azimuthal quantum number: {:?}, Madelung's numbers: {:?}, max number of electrons per subshell: {:?}, electron configuration: {:?}",
            principal_quantum_num,
            electrons_outer_shell,
            azimuthal_num_vector,
            madelung_num_vector,
            max_number_electrons_subshell,
            element_subshell_config
        )
    }

    pub fn create_matrix(&self) {
        let period = self.period as usize;
        let mut dm = DMatrix::from_element(period, period, 2);
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
    pub fn create_molecule(elem1: &Element, elem2: &Element) -> Molecule {
        let elem1_symbol = &elem1.symbol;
        let elem1_valence = &elem1.valence_electrons();

        let elem2_symbol = &elem2.symbol;
        let elem2_valence = &elem2.valence_electrons();

        let mut formula = String::new();

        if elem1_symbol == elem2_symbol {
            formula = format!("{}_{}", &elem1_symbol, *elem1_valence + *elem2_valence);
        } else {
            formula = format!(
                "{}_{}:{}_{}",
                &elem1_symbol, &elem2_valence, &elem2_symbol, &elem1_valence
            );
        }
        let molecule_name = format!("{}-{}\n", &elem1.element, &elem2.element);

        Molecule {
            name: molecule_name,
            formula: formula,
        }
    }
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
pub enum Phase {
    Solid,
    Liquid,
    Gas,
    Plasma,
}

pub enum ElectronConfig {
    LewisDotSymbol,
    OrbitalDiagram,
    SPDFNotation,
    SPDFNotationExpanded,
    NobleGasNotation,
}
