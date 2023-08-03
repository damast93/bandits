use crate::strategies::Strategy;
use crate::bandits::Bandit;

use rand::distributions::{Distribution, Uniform};

pub struct RandomArm { }

impl Strategy for RandomArm {
    fn name(&self) -> String {
        String::from("Random Arm")
    }

    fn run(&self, bandit : &mut Bandit) {

        let n = bandit.num_arms();
        let mut rng = rand::thread_rng();
        let uniform_arm = Uniform::from(0..n);

        while bandit.draws_remaining() > 0 {
            let n = uniform_arm.sample(&mut rng);
            bandit.draw(n);
        }
    }
}
