use std::fmt::Debug;

pub trait Class {
    type Gene: Debug;
    type Phenotype: Debug;
    type Fitness: Debug + PartialEq + PartialOrd;

    const GENES_COUNT: usize;

    fn phenotype(genes: &[Self::Gene]) -> Self::Phenotype;

    fn fitness(genes: &[Self::Gene]) -> Self::Fitness;
}