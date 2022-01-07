use super::utils;

// This problem can be represented as an array 'a' of length 8 where each index
// is the population that is up to a particular day in the breeding cycle.
// So, a[0] are all the lanternfish that will breed today and a[8] is all the
// new lanternfish. Each day the population at a[i] is moved to a[i - 1] and
// the population of a[0] is shifted to both a[6] and a[8]. a is initialised
// as all zeroes and the total population is the sum of the array.

// Using u64 to avoid overflow with the very large numbers produced.

pub fn calculate_lanternfish_population(num_days: u16) -> u64 {
    let ages: Vec<u64> = utils::read_input("day6")
            .split(",")
            .map(|x| x.to_string())
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
    // population is a vector representing the number of lanternfish on
    // each day of the lanternfish creation cycle - there are 9 values stored.
    let mut population = population_array_from_ages(&ages);
    simulate_n_days(num_days, &mut population);
    population.iter().sum()
}


fn population_array_from_ages(ages: &Vec<u64>) -> Vec<u64> {
    let mut population = vec![0; 9];
    for &age in ages {
        population[age as usize] += 1;
    }
    population
}

fn simulate_n_days(n: u16, population: &mut Vec<u64>) {
    for day in 0..n {
        simulate_day(population);
    }
}

fn simulate_day(population: &mut Vec<u64>) {
    let num_new_fish = population[0];
    for i in 1..population.len() {
        population[i - 1] = population[i];
    }
    population[6] += num_new_fish;
    population[8] = num_new_fish;
}



