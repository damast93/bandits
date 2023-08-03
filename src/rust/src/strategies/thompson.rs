use crate::strategies::Strategy;
use crate::bandits::Bandit;
use crate::helpers::argmax;

use rand_distr::{Distribution, Beta};

pub struct Thompson { }

#[derive(Clone)]
struct Pseudocount {
    suc : i32,
    fail : i32
}

fn beta(ps : &Pseudocount) -> f64 {
    let dist = Beta::new(ps.suc as f64, ps.fail as f64).unwrap();
    dist.sample(&mut rand::thread_rng())
}

impl Strategy for Thompson {
    fn name(&self) -> String {
        String::from("Thompson Sampling")
    }

    fn run(&self, bandit : &mut Bandit) {
        let n = bandit.num_arms();

        let mut ps = vec![Pseudocount { suc: 1, fail: 1 }; n];

        while bandit.draws_remaining() > 0 {
            let predictions : Vec<f32> = ps.iter().map(|p| beta(p) as f32).collect();
            let n = argmax(&predictions);
            let r = bandit.draw(n);
            if r == 1.0 {
                ps[n].suc += 1;
            } else if r == 0.0 {
                ps[n].fail += 1;
            } else {
                panic!("Thompson Sampler expects boolean rewards");
            }
        }
    }
}
