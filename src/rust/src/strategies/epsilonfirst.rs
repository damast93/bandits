use crate::{strategies::Strategy, helpers::argmax};
use crate::bandits::Bandit;

use rand::distributions::{Distribution, Uniform};

pub struct EpsilonFirst { 
    pub epsilon : f32
}

#[derive(Clone)]
struct Counter {
    current : f32,
    num_samples : usize
}

fn mean(c : &Counter) -> f32 {
    c.current / (c.num_samples as f32)
}

impl Strategy for EpsilonFirst {
    fn name(&self) -> String {
        String::from(format!("ε-first ({})", self.epsilon))
    }

    fn run(&self, bandit : &mut Bandit) {
        let n = bandit.num_arms();
        let mut payoffs = vec![ Counter { current: 0.0, num_samples: 1 }; n];

        let mut rng = rand::thread_rng();
        let uniform_arm = Uniform::from(0..n);

        let cutoff = ((bandit.draws_remaining() as f32) * (1.0-self.epsilon)) as usize;

        while bandit.draws_remaining() > cutoff {
            let n = uniform_arm.sample(&mut rng);
            
            let payoff = bandit.draw(n);
            payoffs[n].current += payoff;
            payoffs[n].num_samples += 1;
        }
        
        let means : Vec<f32> = payoffs.iter().map(|x| mean(x)).collect();
        let nbest = argmax(&means);

        while bandit.draws_remaining() > 0 {
            bandit.draw(nbest);
        }
    }
}
