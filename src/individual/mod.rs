pub mod class;

use std::fmt::Debug;

pub use class::*;

pub struct Individual<T: Class> {
    genes: Vec<T::Gene>
}

impl<T: Class> Individual<T> {
    pub(super) fn new(genes: Vec<T::Gene>) -> Self {
        Individual {
            genes
        }
    }

    pub fn phenotype(&self) -> T::Phenotype {
        T::phenotype(&self.genes)
    }

    pub fn fitness(&self) -> T::Fitness {
        T::fitness(&self.genes)
    }

    pub fn cross_with<R>(&self, other: &Individual<T>, rule: R) -> Individual<T> 
    where R: FnOnce(&Vec<T::Gene>, &Vec<T::Gene>) -> Vec<T::Gene> {
        let new_genes = rule(&self.genes, &other.genes);

        Individual {
            genes: new_genes
        }
    }
}

impl<T: Class> PartialEq for Individual<T> {
    fn eq(&self, other: &Self) -> bool {
        self.fitness() == other.fitness()
    }
}

impl<T: Class> PartialOrd for Individual<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness().partial_cmp(&other.fitness())
    }
}

impl<T: Class> Debug for Individual<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Individual")
            .field("fitness", &self.fitness())
            .field("phenotype", &self.phenotype())
            .field("genes", &self.genes)
        .finish()
    }
}