use std::env;

mod chromosome;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        println!("Usage: <population size> <selection method> <crossover rate> <mutation rate> <chromosome length>");
        return;
    }

    let pop_size = *&args[1].parse::<usize>().unwrap();
    let selection = *&args[2].parse::<i8>().unwrap();
    let crossover_rate = *&args[3].parse::<f64>().unwrap();
    let mutation_rate = *&args[4].parse::<f64>().unwrap();
    let chromosome_size = *&args[5].parse::<usize>().unwrap();

    println!("Population size: {}\nSelection method (0: proportional, 1: tournament): {}\nCrossover rate: {}\nMutation rate: {}\nChromosome length: {}\n", pop_size, selection, crossover_rate, mutation_rate, chromosome_size);

    let mut population;
    let mut new_population = Vec::new();
    let mut generation = 0;
    let desired_fitness = chromosome_size as i32;

    println!("Initializing population...");
    // get start time
    let now = std::time::Instant::now();
    population = initialize_population(pop_size, chromosome_size);

    let mut best_fitness = 0 as i32;

    println!("Population initialized. Starting evolution...\n");
    // loop until we find a chromosome with fitness of 20
    while best_fitness < desired_fitness {
        while new_population.len() < pop_size as usize {
            // select two parents
            let mut parent1;
            let mut parent2;
            if selection == 0 {
                parent1 = proportional_selection(&population, pop_size);
                parent2 = proportional_selection(&population, pop_size);
            } else {
                parent2 = tournament_selection(&population, pop_size);
                parent1 = tournament_selection(&population, pop_size);
            }

            if parent1.genes.len() == 0 || parent2.genes.len() == 0 {
                continue;
            }

            // random float between 0 and 1
            let mut random = rand::random::<f64>();
            if random < crossover_rate {
                crossover(&mut parent1, &mut parent2, chromosome_size);
            }

            // random float between 0 and 1
            random = rand::random::<f64>();
            if random < mutation_rate {
                mutate(&mut parent1, chromosome_size);
                mutate(&mut parent2, chromosome_size);
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
        if generation % 5000 == 0 {
            println!("Generation: {}, Best Fitness: {}", generation, best.get_fitness());
        }
        generation += 1;
        best_fitness = best.get_fitness();
    }

    let duration = now.elapsed();
    println!("Time to reach desired fitness: {:?}, Avg time per generation {:?}", duration, (duration / generation));
    // avg time per generation
}

// initialize population function
fn initialize_population(pop_size: usize, chromosome_size: usize) -> Vec<chromosome::Chromosome> {
    let mut population: Vec<chromosome::Chromosome> = Vec::new();
    for _ in 0..pop_size {
        let mut cr = chromosome::Chromosome::new();
        // temp array of 20 random genes
        let mut genes: Vec<u32> = Vec::new();
        for _ in 0..chromosome_size {
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

fn tournament_selection(population: &Vec<chromosome::Chromosome>, pop_size: usize) -> chromosome::Chromosome {
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

// weighted roulette wheel selection
fn proportional_selection(population: &Vec<chromosome::Chromosome>, pop_size: usize) -> chromosome::Chromosome {
    let mut sum = 0 as u32;
    for i in 0..pop_size {
        sum += population[i].fitness as u32;
    }

    let r = rand::random::<u32>() % sum;
    let mut count = 0 as u32;
    for j in 0..pop_size {
        count += population[j].fitness as u32;
        if count >= r {
            return population[j].clone();
        }
    }
    chromosome::Chromosome::new()
}

// crossover return both parents
fn crossover(parent1: &mut chromosome::Chromosome, parent2: &mut chromosome::Chromosome, chromosome_size: usize) {
    // random int between 0 and 19
    let r = rand::random::<usize>() % chromosome_size;
    let mut parent1_new: Vec<u32> = Vec::new();
    let mut parent2_new: Vec<u32> = Vec::new();
    // take parent1 genes up to r and parent2 genes after r
    for i in 0..chromosome_size {
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
fn mutate(chromosome: &mut chromosome::Chromosome, chromosome_size: usize) {
    // random int between 0 and 19
    let r = rand::random::<usize>() % chromosome_size;
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
