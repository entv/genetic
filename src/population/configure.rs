use crate::individual::{Class, Individual};

use super::Population;

pub struct ConfigurePopulation<T: Class> {
    individuals: Vec<Individual<T>>,

    count: Option<usize>
}

impl<T: Class> ConfigurePopulation<T> {
    pub fn new() -> Self {
        ConfigurePopulation {
            individuals: Vec::new(),
            count: Default::default()
        }
    }

    pub fn with_count(mut self, count: usize) -> Self {
        self.count = Some(count);

        self
    }
    
    pub fn and_gene_generate_rule<F>(mut self, f: F) -> Self 
    where F: Fn() -> T::Gene {
        for _ in 0..self.count.unwrap_or_else(|| {
            panic!("population count is still unknown")
        }) {
            let mut genes = Vec::with_capacity(T::GENES_COUNT);
            
            for _ in 0..T::GENES_COUNT {
                genes.push(
                    f()
                );
            }

            self.individuals.push(
                Individual::new(genes)
            );
        }

        self
    }

    pub fn configure(self) -> Population<T> {
        Population::new(self.individuals)
    }
}