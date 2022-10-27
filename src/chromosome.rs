pub struct Chromosome {
    pub genes: Vec<u32>,
    pub fitness: i32,
}

impl Chromosome {
    pub fn set_genes(&mut self, genes: Vec<u32>) {
        self.genes = genes;
    }

    pub fn set_gene(&mut self, index: usize, value: u32) {
        self.genes[index] = value;
    }

    pub fn get_gene(&self, index: usize) -> u32 {
        self.genes[index]
    }

    pub fn get_fitness(&self) -> i32 {
        self.fitness
    }

    pub fn set_fitness(&mut self, fitness: i32) {
        self.fitness = fitness;
    }

    pub(crate) fn new() -> Chromosome {
        Chromosome {
            genes: Vec::new(),
            fitness: 0,
        }
    }

    pub fn calculate_fitness(&mut self) {
        let mut fitness = 0;
        for i in 0..self.genes.len() {
            fitness += self.genes[i] as i32;
        }
        self.fitness = fitness;
    }

    // return string representation of chromosome
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..self.genes.len() {
            s.push(self.genes[i].to_string().chars().next().unwrap());
        }
        s
    }

    // clone
    pub fn clone(&self) -> Chromosome {
        let mut c = Chromosome::new();
        c.set_genes(self.genes.clone());
        c.set_fitness(self.fitness.clone());
        c
    }
}

// clone trait
impl Clone for Chromosome {
    fn clone(&self) -> Chromosome {
        self.clone()
    }
}