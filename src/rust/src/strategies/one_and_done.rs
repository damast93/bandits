use crate::strategies::Strategy;
use crate::bandits::Bandit;
use crate::helpers::argmax;

pub struct OneAndDone { }

impl Strategy for OneAndDone {
    fn name(&self) -> String {
        String::from("One and Done")
    }

    fn run(&self, bandit : &mut Bandit) {
        let n = bandit.num_arms();
        let mut payoffs = vec![0.0; n];

        for i in 0..n {
            payoffs[i] = bandit.draw(i);
        }

        let n = argmax(&payoffs);

        while bandit.draws_remaining() > 0 {
            bandit.draw(n);
        }
    }
}
