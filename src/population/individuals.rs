use crate::individual::{Class, Individual};

pub struct Individuals<'population, T: Class> {
    individuals: &'population mut Vec<Individual<T>>
}

impl<'p, T: Class> Individuals<'p, T> {
    pub(super) fn wrap(individuals: &'p mut Vec<Individual<T>>) -> Self {
        Individuals {
            individuals
        }
    }

    pub fn sort(self) -> Self {
        self.individuals.sort_by(|first, second| {
            first.partial_cmp(second).unwrap()
        });

        self
    }

    pub fn take(self, count: usize) -> Self {
        self.individuals.truncate(count);

        self
    }

    pub fn pairs_with<F>(self, f: F) -> IndividualPairs<'p, T, F>
    where F: Fn(usize) -> usize {
        IndividualPairs::wrap(
            self.individuals,
            f
        )
    }
}

pub struct IndividualPairs<'population, T, F>
where T: Class, F: Fn(usize) -> usize {
    individuals: &'population mut Vec<Individual<T>>,
    rule: F
}

impl<'p, T, F> IndividualPairs<'p, T, F> 
where T: Class, F: Fn(usize) -> usize {
    pub(super) fn wrap(individuals: &'p mut Vec<Individual<T>>, rule: F) -> Self {
        IndividualPairs {
            individuals,
            rule
        }
    }

    pub fn and_cross<R>(self, rule: R)
    where R: Fn(&Vec<T::Gene>, &Vec<T::Gene>) -> Vec<T::Gene> {
        let next = &self.rule;
        let len = self.individuals.len();

        for _ in 0..len {
            let first = &self.individuals[next(len)];
            let second = &self.individuals[next(len)];

            self.individuals.push(
                first.cross_with(second, &rule)
            );
        }
    }
}