mod bandits;
mod strategies;
mod helpers;

use strategies::Strategy;

use crate::bandits::{BernoulliFactory,BanditFactory};

struct Statistic {
    mean : f32,
    std : f32
}

fn benchmark(strategy : &(impl Strategy + ?Sized), 
             factory : &(impl BanditFactory + ?Sized),
             num_draws : usize, 
             num_iterations : usize) -> Statistic {
    let mut payoffs = meansd::MeanSD::default();

    for _i in 1..num_iterations {
        let mut bandit = factory.initialize(num_draws);
        strategy.run(&mut bandit);
        payoffs.update(bandit.payoff() as f64);
    }

    Statistic {
        mean : payoffs.mean() as f32,
        std : payoffs.sstdev() as f32
    }
}

fn main() {
    let bandits: Vec<Box<dyn BanditFactory>> = vec![
        Box::new(BernoulliFactory { winning_probabilities : vec![0.5, 0.6, 0.8, 1.0] }),
        Box::new(BernoulliFactory { winning_probabilities : vec![1.0, 0.8, 0.6, 0.5] })
    ];
    
    let strategies = strategies::strategies();

    println!("strategy,bandit,payoff_mean,payoff_std");

    for strategy in strategies.iter() {
        for factory in bandits.iter() {
            let s = benchmark(&**strategy, &**factory, 100, 25000);
            println!("{}, {}, {}, {}", strategy.name(), factory.description(), s.mean, s.std);
        }
    }
}