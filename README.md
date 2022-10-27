# RustGeneticAlgorithm
A very simple genetic algorithm in rust.

# Building
`cargo build --release`

# Running
`./genetic_algorithm <population size> <selection method> <crossover rate> <mutation rate> <chromosome length>`

## Example run
```
user$ ./genetic_algorithm 100 1 .05 .01 100
Population size: 100
Selection method (0: proportional, 1: tournament): 1
Crossover rate: 0.05
Mutation rate: 0.01
Chromosome length: 100

Initializing population...
Population initialized. Starting evolution...

Generation: 0, Best Fitness: 62

Time to reach desired fitness: 19.410707ms, Avg time per generation 28.629µs
Best generation was 678 with a fitness of 100
```
