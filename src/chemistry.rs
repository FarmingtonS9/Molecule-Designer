//Imports
use serde::Deserialize;
use std::fmt::Display;

//Constants
const ENERGY_SUBLEVEL: [&str; 4] = ["s", "p", "d", "f"];

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct Atom {
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
    #[serde(rename = "Radioactive", deserialize_with = "csv::invalid_option")]
    pub radioactive: Option<String>
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Element: {}, Symbol: {}, Atomic Number: {}, Protons: {}, Neutrons: {}, Electrons: {}, Radioactive: {:?}",
            self.element,
            self.symbol,
            self.atomic_num,
            self.num_of_protons,
            self.num_of_neutrons,
            self.num_of_electrons,
            self.radioactive,
        )
    }
}

impl Default for Atom {
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
            radioactive: Some(String::from("None"))
        }
    }
}

impl Atom {
    //This outputs the number of valence electrons.
    //Useful for determining reactions
    //Needs to reworking
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
            2 => valence = { electron - 2 },
            3 => valence = { electron - 10 },
            _ => todo!(),
        }
        valence
    }

    pub fn oxidation_states(&self) {
        todo!()
    }

    pub fn list() {
        
    }
}

#[derive(Debug)]
pub struct Molecule {
    name: String,
    atom: Vec<Atom>,
}

impl Molecule {
    fn create_molecule(&self) {
        todo!()
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
