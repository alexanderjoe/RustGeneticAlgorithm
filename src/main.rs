mod chromosome;

static CHROMOSOME_SIZE: usize = 100;

fn main() {
    // an array of chromosomes of size 100
    let mut population: Vec<chromosome::Chromosome> = Vec::new();
    let mut new_population: Vec<chromosome::Chromosome> = Vec::new();
    // desired fitness int 20
    let desired_fitness = CHROMOSOME_SIZE as i32;
    // population size
    let pop_size = 100;
    // percent crossover
    let crossover_rate = 0.5;
    // percent mutation
    let mutation_rate = 0.01;
    // current generation
    let mut generation = 0;

    println!("Initializing population...");
    // get start time
    let now = std::time::Instant::now();
    population = initialize_population(pop_size);

    // print each chromosome in the population
    // for i in 0..population.len() {
    //     println!("{}: {}", i, population[i].to_string());
    // }

    let mut best_fitness = 0 as i32;
    let mut best_chromosome = chromosome::Chromosome::new();

    println!("Population initialized. Starting evolution...");
    // loop until we find a chromosome with fitness of 20
    while best_fitness < desired_fitness {
        while new_population.len()  < pop_size as usize {
            // select two parents
            let mut parent1 = select_parent(&population, pop_size);
            let mut parent2 = select_parent(&population, pop_size);

            // random float between 0 and 1
            let mut random = rand::random::<f64>();
            if random < crossover_rate {
                crossover(&mut parent1, &mut parent2);
            }

            // random float between 0 and 1
            random = rand::random::<f64>();
            if random < mutation_rate {
                mutate(&mut parent1);
                mutate(&mut parent2);
            }

            // add parents to new population
            new_population.push(parent1);
            new_population.push(parent2);
        }
        // replace old population with new population
        population.clear();
        population = new_population.to_vec();
        new_population.clear();

        let best = get_most_fit(&mut population);
        // print best
        println!("Generation {}: {}", generation, best.to_string());
        println!("Generation: {}, Best Fitness: {}", generation, best.get_fitness());
        generation += 1;
        best_fitness = best.get_fitness();
    }

    let duration = now.elapsed();
    println!("{:?}", duration);
}

// initialize population function
fn initialize_population(pop_size: usize) -> Vec<chromosome::Chromosome> {
    let mut population: Vec<chromosome::Chromosome> = Vec::new();
    for _ in 0..pop_size {
        let mut cr = chromosome::Chromosome::new();
        // temp array of 20 random genes
        let mut genes: Vec<u32> = Vec::new();
        for _ in 0..CHROMOSOME_SIZE {
            // random int 0-1
            let r = rand::random::<u32>() % 2;
            genes.push(r);
        }
        cr.set_genes(genes);
        cr.calculate_fitness();
        population.push(cr);
    }
    population
}

fn select_parent(population: &Vec<chromosome::Chromosome>, pop_size: usize) -> chromosome::Chromosome {
    // use tournament selection
    let mut tournament: Vec<chromosome::Chromosome> = Vec::new();
    // add random chromosomes to the tournament
    for _ in 0..5 {
        // random int between 0 and pop_size
        let r = rand::random::<usize>() % pop_size;
        tournament.push(population[r].clone());
    }
    // sort the tournament by fitness
    tournament.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    // return the best chromosome
    tournament[0].clone()
}

// crossover return both parents
fn crossover(parent1: &mut chromosome::Chromosome, parent2: &mut chromosome::Chromosome) {
    // random int between 0 and 19
    let r = rand::random::<usize>() % CHROMOSOME_SIZE;
    let mut parent1_new: Vec<u32> = Vec::new();
    let mut parent2_new: Vec<u32> = Vec::new();
    // take parent1 genes up to r and parent2 genes after r
    for i in 0..CHROMOSOME_SIZE {
        if i < r {
            parent1_new.push(parent1.genes[i]);
            parent2_new.push(parent2.genes[i]);
        } else {
            parent1_new.push(parent2.genes[i]);
            parent2_new.push(parent1.genes[i]);
        }
    }
    parent1.set_genes(parent1_new);
    parent2.set_genes(parent2_new);
}

// mutation
fn mutate(chromosome: &mut chromosome::Chromosome) {
    // random int between 0 and 19
    let r = rand::random::<usize>() % CHROMOSOME_SIZE;
    // flip the gene
    if chromosome.get_gene(r) == 0 {
        chromosome.set_gene(r, 1);
    } else {
        chromosome.set_gene(r, 0);
    }
}


// get populations best fitness
fn get_most_fit(population: &mut Vec<chromosome::Chromosome>) -> chromosome::Chromosome {
    let mut best_fitness = 0;
    let mut best_chromosome = chromosome::Chromosome::new();
    for i in 0..population.len() {
        population[i].calculate_fitness();
        if population[i].get_fitness() > best_fitness {
            best_fitness = population[i].get_fitness();
            best_chromosome = population[i].clone();
        }
    }
    best_chromosome
}
