pub mod configure;
pub mod individuals;

pub use configure::*;
pub use individuals::*;

use std::fmt::Debug;

use crate::individual::{Class, Individual};

pub struct Population<T: Class> {
    individuals: Vec<Individual<T>>
}

impl<T: Class> Population<T> {
    pub(super) fn new(individuals: Vec<Individual<T>>) -> Self {
        Population {
            individuals
        }
    }

    pub fn best(&self) -> &Individual<T> {
        self.individuals.iter()
            .min_by(|&fst, &scnd| {
                fst.partial_cmp(scnd).unwrap()
            })
            .unwrap()
    }

    pub fn individuals(&mut self) -> Individuals<T> {
        Individuals::wrap(&mut self.individuals)
    }
}

impl<T: Class> Debug for Population<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Population")
            .field("individuals", &self.individuals)
        .finish()
    }
}