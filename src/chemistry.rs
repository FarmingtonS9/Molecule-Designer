//Imports
use std::fmt::Display;

use serde::Deserialize;

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
}

impl Display for Atom {
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
        }
    }
}

impl Atom {
    pub fn diff_from_full_valence(&self) -> i32 {
        let num_of_valence_electrons = self.calculate_valence_electrons();

        let difference = 8 - num_of_valence_electrons;
        difference
    }

    pub fn calculate_valence_electrons(&self) -> i32 {
        let num_of_electrons = self.num_of_electrons;

        let num_of_valence_electrons = match self.period {
            1 => match self.atomic_num {
                1 => num_of_electrons,
                _ => 2,
            },
            2 => num_of_electrons - 2,
            3 => num_of_electrons - 10,
            4 => num_of_electrons - 18,
            5 => num_of_electrons - 36,
            6 => num_of_electrons - 54,
            7 => num_of_electrons - 86,
            _ => num_of_electrons - 118,
        };

        num_of_valence_electrons
    }

    pub fn electron_configuration(&self) {
        let period = self.period;
        let mut electron_configuration = "1s1";
        let s_max = "s2";
        let p_max = "p6";
        let d_max = "d10";
        let f_max = "f14";

        let electron_configuration = match period {
            1 => {
                electron_configuration = format!("{}{}{}", period, ENERGY_SUBLEVEL[0], self.atomic_num).as_str()
            }
            _ => electron_configuration = "1s1",
        };
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