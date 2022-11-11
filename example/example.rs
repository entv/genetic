use genetic::{
    individual::{Class, Individual}, population::{ConfigurePopulation, Population},
};

pub struct Rat;

impl Class for Rat {
    type Gene = f32;
    type Phenotype = Vec<f32>;
    type Fitness = f32;

    const GENES_COUNT: usize = 5;

    fn phenotype(genes: &[Self::Gene]) -> Self::Phenotype {
        let phenotype_len = 5;
        let mut phenotype = Vec::with_capacity(phenotype_len);

        for &gene in genes {
            phenotype.push(gene)
        }

        phenotype
    }

    fn fitness(genes: &[Self::Gene]) -> Self::Fitness {
        let phenotype = Self::phenotype(genes);

        phenotype.iter()
            .map(|&phenotype_num| {
                phenotype_num * phenotype_num
            })
            .sum::<f32>()
            .sqrt();
    }
}

fn main() {
    let mut population: Population<Rat> = ConfigurePopulation::new()
        .with_count(10)
        .and_gene_generate_rule(|| {
            random_number::random_ranged(0.0..10.0)
        })
        .configure();
    
    println!("{:#?}", population.best().fitness());

    for _ in 0..1_000_000 {
        population.individuals()
            
            .sort()
            
            .take(5)

            .pairs_with(|count| {
                random_number::random_ranged(0..count)
            })

            .and_cross(|first, second| {
                let mut genes = Vec::with_capacity(Rat::GENES_COUNT);

                let cross_point = random_number::random_ranged(0..Rat::GENES_COUNT);

                for gene_number in 0..cross_point {
                    genes.push(first[gene_number]);
                }

                for gene_number in cross_point..Rat::GENES_COUNT {
                    genes.push(second[gene_number]);
                }

                if random_number::random_ranged(0..100) < 20 {
                    let gene_number_to_mutate = random_number::random_ranged(0..genes.len());

                    let gene_to_mutate = &mut genes[gene_number_to_mutate];

                    *gene_to_mutate += random_number::random_ranged(-gene_to_mutate.abs() * 0.1..gene_to_mutate.abs() * 0.1);
                }

                genes
            });
        
        println!("{:#?}", population.best().fitness()); 
    }
}