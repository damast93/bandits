use crate::{strategies::Strategy, helpers::argmax};
use crate::bandits::Bandit;

use rand::distributions::{Distribution, Uniform};

pub struct EpsilonGreedy { 
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

impl Strategy for EpsilonGreedy {
    fn name(&self) -> String {
        String::from(format!("ε-greedy ({})", self.epsilon))
    }

    fn run(&self, bandit : &mut Bandit) {
        let n = bandit.num_arms();
        let mut payoffs = vec![ Counter { current: 0.0, num_samples: 1 }; n];

        let mut rng = rand::thread_rng();
        let uniform_arm = Uniform::from(0..n);

        while bandit.draws_remaining() > 0 {
            let p = rand::random::<f32>();
            if p < self.epsilon {
                let n = uniform_arm.sample(&mut rng);
                
                let payoff = bandit.draw(n);
                payoffs[n].current += payoff;
                payoffs[n].num_samples += 1;
            } else {
                let means : Vec<f32> = payoffs.iter().map(|x| mean(x)).collect();
                let n = argmax(&means);
                let payoff = bandit.draw(n);
                payoffs[n].current += payoff;
                payoffs[n].num_samples += 1;
            }
        }
    }
}
