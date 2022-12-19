//Properties and behaviours of electrons
//May move this into a "Particle.rs" file later.

pub mod Electron {
    pub struct Electron {
        spin: bool, //Spin state, 0 is up, 1 is down.
    }

    impl Default for Electron {
        fn default(&self) {
            Electron { spin: 0 }
        }
    }

    impl Electron {}
}
