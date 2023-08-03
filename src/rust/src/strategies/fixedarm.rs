use crate::strategies::Strategy;
use crate::bandits::Bandit;

use rand::distributions::{Distribution, Uniform};

pub struct FixedArm { }

impl Strategy for FixedArm {
    fn name(&self) -> String {
        String::from("Fixed Random Arm")
    }

    fn run(&self, bandit : &mut Bandit) {

        let n = bandit.num_arms();
        let mut rng = rand::thread_rng();
        let uniform_arm = Uniform::from(0..n);
        let n = uniform_arm.sample(&mut rng);

        while bandit.draws_remaining() > 0 {
            bandit.draw(n);
        }
    }
}