#![allow(unused)]

//Imports
use serde::Deserialize;
use std::fmt::Display;

//Constants
const ENERGY_SUBLEVEL: [&str; 4] = ["s", "p", "d", "f"];

//Traits (shared behaviours/properties)
pub trait Atom {
    //Electron configuration of atom
    fn electronic_structure() {}
}

//Structs (data)
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

    pub fn list() {}
}

//Impl for different representations of electron configurations
impl Element {
    //Lewis dot diagram for element.
    //Make this associated function more robust
    pub fn lewis_dot_symbol(&self) -> String {
        let dot = "•";
        let colon = ":";

        let diagram = format!("{}{}{}", dot, &self.symbol, dot);
        diagram
    }
}

//Private associated functions
impl Element {}

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
