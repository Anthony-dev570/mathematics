use crate::chemistry::element::classification::Classification;
use crate::chemistry::element::classification::Classification::*;
use crate::chemistry::element::Element;
use crate::chemistry::phase::Phase;
use crate::chemistry::temperature::Temperature;

impl<'a> Element<'a> {
    ///https://en.wikipedia.org/wiki/List_of_chemical_elements
    pub const PERIODIC_TABLE: &'static [Element<'static>] = &[
        Element {
            atomic_number: 1,
            symbol: "H",
            name: "Hydrogen",
            group: 1,
            period: 1,
            block: 's',
            atomic_weight: 1.0080,
            density: 0.00008988,
            melting_point: Some(Temperature::from_kelvin(14.01)),
            boiling_point: Temperature::from_kelvin(20.28),
            specific_heat_capacity: 14.304,
            electro_negativity: Some(2.2),
            phase: Phase {},
            classification: ReactiveNonmetal
        },
        Element {
            atomic_number: 2,
            symbol: "He",
            name: "Helium",
            group: 18,
            period: 1,
            block: 's',
            atomic_weight: 4.0026,
            density: 0.0001785,
            melting_point: None,
            boiling_point: Temperature::from_kelvin(4.22),
            specific_heat_capacity: 5.193,
            electro_negativity: None,
            phase: Phase {},
            classification: NobleGas,
        },
        Element {
            atomic_number: 3,
            symbol: "Li",
            name: "Lithium",
            group: 1,
            period: 2,
            block: 's',
            atomic_weight: 6.94,
            density: 0.534,
            melting_point: None,
            boiling_point: Temperature::from_kelvin(1560.0),
            specific_heat_capacity: 0.0,
            electro_negativity: None,
            phase: Phase {},
            classification: Alkali
        }
    ];

    pub fn element_from_atomic_number(atomic_number: u8) -> &'static Element<'static> {
        &Self::PERIODIC_TABLE[atomic_number as usize - 1]
    }

    pub fn atomic_number(&self) -> u8 {
        self.atomic_number
    }
    pub fn symbol(&self) -> &'a str {
        self.symbol
    }
    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn group(&self) -> u8 {
        self.group
    }
    pub fn period(&self) -> u8 {
        self.period
    }
    pub fn block(&self) -> char {
        self.block
    }
    pub fn atomic_weight(&self) -> f64 {
        self.atomic_weight
    }
    pub fn density(&self) -> f64 {
        self.density
    }
    pub fn melting_point(&self) -> Option<Temperature> {
        self.melting_point
    }
    pub fn boiling_point(&self) -> Temperature {
        self.boiling_point
    }
    pub fn specific_heat_capacity(&self) -> f64 {
        self.specific_heat_capacity
    }
    pub fn electro_negativity(&self) -> Option<f64> {
        self.electro_negativity
    }
    pub fn phase(&self) -> &Phase {
        &self.phase
    }
    pub fn classification(&self) -> &Classification {
        &self.classification
    }
}