use csv::Reader;
use petgraph::dot::Dot;
use petgraph::*;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::io::Write;

const GENERATE_FILE_FOR_CLOUD: &str =
    r"G:\Cloud\Google Drive\Learning\Reading\Manufacturing\wind_turbine.txt";

const PROCESS_PLACEHOLDER: &str = "Find name of process";
const MANUFACTURED: &str = "Manufactured";
const MINED: &str = "Mined";

const OCTET_NUMBER: u32 = 8;
const ENERGY_SUBLEVEL: [&str; 4] = ["s", "p", "d", "f"];
const PRINCIPLE_ENERGY_LEVEL: [i32; 4] = [1, 2, 3, 4];

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
struct Atom {
    #[serde(rename = "Element")]
    element: String,
    #[serde(rename = "Symbol")]
    symbol: String,
    #[serde(rename = "AtomicNumber")]
    atomic_num: i32,
    #[serde(rename = "AtomicMass")]
    atomic_mass: f32,
    #[serde(rename = "NumberofProtons")]
    num_of_protons: i32,
    #[serde(rename = "NumberofNeutrons")]
    num_of_neutrons: i32,
    #[serde(rename = "NumberofElectrons")]
    num_of_electrons: i32,
    #[serde(rename = "Period")]
    period: i32,
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
    fn diff_from_full_valence(&self) -> i32 {
        let num_of_valence_electrons = self.calculate_valence_electrons();

        let difference = 8 - num_of_valence_electrons;
        difference
    }

    fn calculate_valence_electrons(&self) -> i32 {
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

    fn electron_configuration(&self) {
        let period = self.period;
        let mut electron_configuration = "1s1";
        let s_max = "s2";
        let p_max = "p6";
        let d_max = "d10";
        let f_max = "f14";

        let electron_configuration = match period {
            1 => {
                electron_configuration =
                    &(period.to_string() + ENERGY_SUBLEVEL[0] + &self.atomic_num.to_string())
            }
            _ => electron_configuration = "1s1",
        };
    }
}

#[derive(Debug)]
struct Particles {
    protons: i32,
    neutrons: i32,
    electrons: i32,
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

fn main() {
    println!("Hello, world!");
    println!("I am going to create a material and process tracker.");
    println!("Why? Because I want to keep track of materials and their associated processes");

    //wind_turbine_graph();

    let file_path = r"..\material-and-process-tracker\Periodic Table of Elements.csv";

    if let Ok(atom_collection) = read_csv_data(file_path) {
        let mut vector_of_elements: Vec<String> = Vec::new();
        let mut element_particles: Vec<Particles> = Vec::new();

        //Vec of Atom now works
        for atom in &atom_collection {
            println!("{}", atom)
        }

        let names_from_atom_collection = atom_collection.clone();
        for name in names_from_atom_collection {
            vector_of_elements.push(name.element)
        }
        //println!("{:?}", element_names);

        let number_of_particles_atom_collection = atom_collection.clone();
        for particles in number_of_particles_atom_collection {
            let protons = particles.num_of_protons;
            let neutrons = particles.num_of_neutrons;
            let electrons = particles.num_of_electrons;
            element_particles.push(Particles {
                protons,
                neutrons,
                electrons,
            })
        }
        //println!("{}", element_particles)

        let carbon = atom_from_atom_collection("Carbon", &vector_of_elements, &atom_collection);
        println!("{}", &carbon);

        let sodium = atom_from_atom_collection("Sodium", &vector_of_elements, &atom_collection);
        let chlorine = atom_from_atom_collection("Chlorine", &vector_of_elements, &atom_collection);
        println!("{}, {}", sodium, chlorine);

        let silicon = atom_from_atom_collection("Silicon", &vector_of_elements, &atom_collection);
        println!("{}", silicon.diff_from_full_valence());

        let aluminium =
            atom_from_atom_collection("Aluminium", &vector_of_elements, &atom_collection);
        let oxygen = atom_from_atom_collection("Oxygen", &vector_of_elements, &atom_collection);

        println!("{}", aluminium);
        println!("{}", oxygen);
        println!(
            "{}:{}",
            aluminium.calculate_valence_electrons(),
            oxygen.diff_from_full_valence()
        );
        molecule_designer(aluminium, oxygen);

        let mut max_electrons: i32;
        let mut num_of_orbitals: i32;

        for n in PRINCIPLE_ENERGY_LEVEL {
            num_of_orbitals = i32::pow(n, 2);
            max_electrons = 2 * i32::pow(n, 2);
            println!("{}, {}", num_of_orbitals, max_electrons)
        }

        carbon.electron_configuration();

        //wind_turbine_graph()
    }
}

fn read_csv_data(file_path: &str) -> Result<Vec<Atom>, Box<dyn Error>> {
    let mut atom_collection: Vec<Atom> = Vec::new();
    let mut atom: Atom;

    let mut rdr = Reader::from_path(file_path).expect("Could not open csv file");

    for result in rdr.deserialize() {
        match result {
            Ok(atom) => atom_collection.push(atom),
            Err(err) => panic!(),
        };
    }
    Ok(atom_collection)
}

fn find_element_in_atom_collection(element_name: &str, vector_of_elements: &Vec<String>) -> usize {
    let element_name = element_name.to_string();
    let vector_of_elements = vector_of_elements;
    let index = vector_of_elements
        .iter()
        .position(|r| r.as_str() == element_name)
        .expect("Cannot find index");
    index.try_into().unwrap()
}

fn atom_from_atom_collection<'a>(
    element_name: &'a str,
    vector_of_elements: &'a Vec<String>,
    atom_collection: &'a Vec<Atom>,
) -> &'a Atom {
    let element_index = find_element_in_atom_collection(element_name, vector_of_elements);
    let atom = &atom_collection[element_index];
    atom
}

fn molecule_designer(first_element: &Atom, second_element: &Atom) {
    let first_diff_from_full_valence = first_element.diff_from_full_valence();
    let second_diff_from_full_valence = second_element.calculate_valence_electrons();

    let num_of_first_elements = second_diff_from_full_valence;
    let num_of_second_elements = first_diff_from_full_valence;

    println!(
        "Molecule designed: {}{}{}{}",
        first_element.symbol, num_of_first_elements, second_element.symbol, num_of_second_elements
    )
}

fn wind_turbine_graph() -> () {
    let mut graph = Graph::<&str, &str>::new();
    let wind_turbine = graph.add_node("Wind Turbine");

    let nacelle = graph.add_node("Nacelle");
    let bearings = graph.add_node("Bearings");
    let high_speed_coupling = graph.add_node("High-Speed Coupling");
    let low_speed_shaft = graph.add_node("Low-Speed Shaft");
    let yaw_drive = graph.add_node("Yaw Drive");
    let hydraulics = graph.add_node("Hydraulics");
    let cooling_system = graph.add_node("Cooling System");
    let electronics = graph.add_node("Electronics"); //Catch-all for various electrical systems. E.g variable-speed electronics
    let industrial_process_control = graph.add_node("Industrial Process Control");

    let blade = graph.add_node("Blade");
    let gearbox = graph.add_node("Gearbox");
    let generator = graph.add_node("Generator");
    let hub = graph.add_node("Hub");
    let tower = graph.add_node("Tower");

    let aluminium = graph.add_node("Aluminium");
    let steel = graph.add_node("Steel");
    let carbon_fibre = graph.add_node("Carbon Fibre");
    let glass_reinforced_plastic = graph.add_node("Glass-Reinforced Plastic");
    let copper = graph.add_node("Copper");
    let prestressed_concrete = graph.add_node("Pre-Stressed Concrete");
    let concrete = graph.add_node("Concrete");
    let carbon_steel = graph.add_node("Carbon Steel");

    let alumina = graph.add_node("Alumina");
    let cryolite = graph.add_node("Cryolite");

    let bauxite = graph.add_node("Bauxite");
    let iron_ore = graph.add_node("Iron Ore");

    let caustic_soda = graph.add_node("Caustic Soda (NaOH)");
    let grout = graph.add_node("Grout");

    let ground_lime = graph.add_node("Ground Lime");
    let silica_glass = graph.add_node("Silica Glass");
    let sodium_carbonate = graph.add_node("Sodium Carbonate");
    let boron = graph.add_node("Boron");
    let quartz = graph.add_node("Quartz");

    let brine = graph.add_node("Brine");
    let water = graph.add_node("Water");

    let sodium_chloride = graph.add_node("Sodium Chloride");
    let sodium_bicarbonate = graph.add_node("Sodium Bicarbonate");
    let carbon_dioxide = graph.add_node("Carbon Dioxide");

    let seawater = graph.add_node("Seawater");
    let rock_salt = graph.add_node("Rock Salt");

    let iron = graph.add_node("Iron");
    let coal = graph.add_node("Coal");
    let lime = graph.add_node("Lime");

    let polyacrylonitrile = graph.add_node("Polyacrylonitrile");
    let acrylonitrile = graph.add_node("Acrylonitrile");

    let ethylene_cyanohydrin = graph.add_node("Ethylene Cyanohydrin");

    let e_glass = graph.add_node("E-Glass (Borosilicate Glass)");
    let s_glass = graph.add_node("S-Glass (Magnesium Alumino-Silicate)");
    let epoxy_resin = graph.add_node("Epoxy Resin");

    let sodium_silicate = graph.add_node("Sodium Silicate");
    let magnesium_aluminate = graph.add_node("Magnesium Aluminate");

    let polypropylene = graph.add_node("Polypropylene");
    let air = graph.add_node("Air");
    let ammonia = graph.add_node("Ammonia");

    let bpa = graph.add_node("Bisphenol A (BPA)");
    let epichlorohydrin = graph.add_node("Epichlorohydrin");

    let dichlorohydrin = graph.add_node("Dichlorohydrin");
    let glycerol = graph.add_node("Glycerol");

    let allyl_chloride = graph.add_node("Allyl Chloride");

    let triglyceride = graph.add_node("Triglyceride");

    let acetone = graph.add_node("Acetone");
    let phenol = graph.add_node("Phenol");
    let cumene = graph.add_node("Cumene");

    let benzene = graph.add_node("Benzene");

    let propylene = graph.add_node("Propylene");
    let naphtha = graph.add_node("Naphtha");
    let propane = graph.add_node("Propane");

    let pygas = graph.add_node("Pygas");
    let crude_oil = graph.add_node("Crude Oil");

    let nitrogen = graph.add_node("Nitrogen");
    let hydrogen = graph.add_node("Hydrogen");

    let methane = graph.add_node("Methane");
    let natural_gas = graph.add_node("Natural Gas");

    let limestone = graph.add_node("Limestone");

    let copper_oxide = graph.add_node("Copper Oxide");
    let copper_sulphide = graph.add_node("Copper Sulphide");
    let copper_ore = graph.add_node("Copper Ore");

    graph.extend_with_edges(&[
        //Components of turbine
        (blade, wind_turbine, MANUFACTURED),
        (nacelle, wind_turbine, MANUFACTURED),
        (gearbox, nacelle, MANUFACTURED),
        (generator, nacelle, MANUFACTURED),
        (hub, wind_turbine, MANUFACTURED),
        (tower, wind_turbine, MANUFACTURED),
    ]);

    graph.extend_with_edges(&[
        //Blade materials
        (glass_reinforced_plastic, blade, MANUFACTURED),
        (carbon_fibre, blade, MANUFACTURED),
        (steel, blade, MANUFACTURED),
    ]);

    graph.extend_with_edges(&[
        //Nacelle
        (bearings, nacelle, MANUFACTURED),
        (high_speed_coupling, nacelle, MANUFACTURED),
        (low_speed_shaft, nacelle, MANUFACTURED),
        (yaw_drive, nacelle, MANUFACTURED),
        (hydraulics, nacelle, MANUFACTURED),
        (cooling_system, nacelle, MANUFACTURED),
        (electronics, nacelle, MANUFACTURED),
        (industrial_process_control, nacelle, MANUFACTURED),
    ]);

    graph.extend_with_edges(&[(carbon_steel, low_speed_shaft, MANUFACTURED)]);

    graph.extend_with_edges(&[
        //Gearbox materials
        (aluminium, gearbox, MANUFACTURED),
        (steel, gearbox, MANUFACTURED),
    ]);

    graph.extend_with_edges(&[
        //Generator materials
        (copper, generator, MANUFACTURED),
        (steel, generator, MANUFACTURED),
    ]);

    graph.extend_with_edges(&[
        //Hub materials
        (steel, hub, MANUFACTURED),
    ]);

    graph.extend_with_edges(&[
        //Tower materials
        (steel, tower, MANUFACTURED),
        (prestressed_concrete, tower, MANUFACTURED),
    ]);

    //Aluminium
    graph.extend_with_edges(&[
        (alumina, aluminium, "Herault-Holt Process"),
        (cryolite, aluminium, "Herault-Holt Process"),
    ]);

    //Alumina graph
    graph.extend_with_edges(&[
        (bauxite, alumina, "Bayer Process"),
        (caustic_soda, alumina, "Bayer Process"),
    ]);

    //Caustic Soda (Sodium Hydroxide)
    graph.extend_with_edges(&[(brine, caustic_soda, "Chlor-Alkali Process (Electrolysis)")]);

    //Carbon fibre
    graph.extend_with_edges(&[
        (polyacrylonitrile, carbon_fibre, PROCESS_PLACEHOLDER),
        (acrylonitrile, polyacrylonitrile, "Vinyl Polymerisation"),
    ]);

    //Lime
    graph.extend_with_edges(&[(limestone, lime, "Calcination")]);

    //Acrylonitrile
    graph.extend_with_edges(&[
        (polypropylene, acrylonitrile, "Ammoxidation"),
        (air, acrylonitrile, "Ammoxidation"),
        (ammonia, acrylonitrile, "Ammoxidation"),
    ]);

    //Polypropylene
    graph.extend_with_edges(&[(propylene, polypropylene, "Polymerisation")]);

    //Propylene
    graph.extend_with_edges(&[
        (naphtha, propylene, "Steam Cracking"),
        (propane, propylene, "Propane Dehydrogenation"),
    ]);

    //Naphtha
    graph.extend_with_edges(&[(crude_oil, naphtha, "Refining")]);

    //Propane
    graph.extend_with_edges(&[
        (crude_oil, propane, "Associated Gasses"),
        (natural_gas, propane, "Processed"),
    ]);

    //Ammonia
    graph.extend_with_edges(&[
        (nitrogen, ammonia, "Haber-Bosch Process"),
        (hydrogen, ammonia, "Haber-Bosch Process"),
    ]);

    //Hydrogen
    graph.extend_with_edges(&[(methane, hydrogen, "Steam-Methane Reforming")]);

    //Methane
    graph.extend_with_edges(&[(natural_gas, methane, PROCESS_PLACEHOLDER)]);

    //Cumene
    graph.extend_with_edges(&[
        (propylene, cumene, "Alkylation"),
        (benzene, cumene, "Alkylation"),
    ]);

    //Benzene
    graph.extend_with_edges(&[(pygas, benzene, "Hydrotreating")]);

    //Pygas
    graph.extend_with_edges(&[
        (naphtha, pygas, "Steam Cracking"),
        (naphtha, pygas, "Catalytic Reforming"),
        (propane, pygas, "Steam Cracking"),
    ]);

    //Pre-stressed concrete
    graph.extend_with_edges(&[
        (steel, prestressed_concrete, MANUFACTURED),
        (grout, prestressed_concrete, MANUFACTURED),
        (concrete, prestressed_concrete, MANUFACTURED),
    ]);

    //Steel
    graph.extend_with_edges(&[
        (iron, steel, "Blast Oxygen Steelmaking"),
        (coal, steel, "Blast Oxygen Steelmaking"),
        (lime, steel, "Blast Oxygen Steelmaking"),
    ]);

    //Iron
    graph.extend_with_edges(&[(iron_ore, iron, MINED)]);

    //Copper
    graph.extend_with_edges(&[
        (copper_oxide, copper, "Hydrometallurgy"),
        (copper_sulphide, copper, "Pyrometallurgy"),
        (copper_ore, copper_oxide, "Crushing"),
        (copper_ore, copper_sulphide, "Crushing"),
    ]);

    //Glass-Reinforced Fibre
    graph.extend_with_edges(&[
        (e_glass, glass_reinforced_plastic, MANUFACTURED),
        (s_glass, glass_reinforced_plastic, MANUFACTURED),
        (epoxy_resin, glass_reinforced_plastic, MANUFACTURED),
    ]);

    //E-Glass
    graph.extend_with_edges(&[
        (boron, e_glass, PROCESS_PLACEHOLDER),
        (silica_glass, e_glass, PROCESS_PLACEHOLDER),
        (sodium_carbonate, e_glass, PROCESS_PLACEHOLDER),
        (ground_lime, e_glass, PROCESS_PLACEHOLDER),
    ]);

    //S/S2-Glass
    graph.extend_with_edges(&[
        (magnesium_aluminate, s_glass, PROCESS_PLACEHOLDER),
        (sodium_silicate, s_glass, PROCESS_PLACEHOLDER),
    ]);

    //Sodium Carbonate
    graph.extend_with_edges(&[
        (sodium_chloride, sodium_carbonate, "Solvay Process"),
        (ammonia, sodium_carbonate, "Solvay Process"),
        (limestone, sodium_carbonate, "Solvay Process"),
    ]);

    //Sodium Chloride
    graph.extend_with_edges(&[
        (seawater, sodium_chloride, "Evaporation"),
        (rock_salt, sodium_chloride, MINED),
        (brine, sodium_chloride, "Evaporation"),
    ]);

    //Silica glass
    graph.extend_with_edges(&[(quartz, silica_glass, PROCESS_PLACEHOLDER)]);

    //Epoxy resin
    graph.extend_with_edges(&[
        (bpa, epoxy_resin, PROCESS_PLACEHOLDER),
        (epichlorohydrin, epoxy_resin, PROCESS_PLACEHOLDER),
    ]);

    //BPA
    graph.extend_with_edges(&[
        (acetone, bpa, PROCESS_PLACEHOLDER),
        (phenol, bpa, PROCESS_PLACEHOLDER),
        (cumene, acetone, PROCESS_PLACEHOLDER),
        (cumene, phenol, PROCESS_PLACEHOLDER),
    ]);

    //Epichlorohydrin
    graph.extend_with_edges(&[
        (glycerol, epichlorohydrin, "Halogenation"),
        (propylene, allyl_chloride, "Chlorination"),
        (allyl_chloride, epichlorohydrin, "Hypochlorous Acid"),
    ]);

    //Glycerol
    graph.extend_with_edges(&[
        (triglyceride, glycerol, "Saponification"),
        (caustic_soda, glycerol, "Saponification"),
    ]);

    //output to file
    let mut file = std::fs::File::create("wind_turbine.txt").expect("Failed to create file");
    let mut cloud_file =
        std::fs::File::create(GENERATE_FILE_FOR_CLOUD).expect("Failed to create file");

    cloud_file
        .write_all(Dot::new(&graph).to_string().as_bytes())
        .expect("Failed to export cloud file");
    file.write_all(Dot::new(&graph).to_string().as_bytes())
        .expect("Failed to export to text file");

    //println!("{}", Dot::new(&graph));
}
