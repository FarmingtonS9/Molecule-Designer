//File for chemical models needed

pub trait MolecularOrbitalTheory {}

pub trait ValenceBondTheory: MolecularOrbitalTheory {}

pub trait VSEPR: ValenceBondTheory {}
