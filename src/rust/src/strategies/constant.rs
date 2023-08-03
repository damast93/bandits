use crate::strategies::Strategy;
use crate::bandits::Bandit;
pub struct Constant { pub n : usize }

impl Strategy for Constant {
    fn name(&self) -> String {
        String::from(format!("Constant {}", self.n))
    }

    fn run(&self, bandit : &mut Bandit) {
        while bandit.draws_remaining() > 0 {
            bandit.draw(self.n);
        }
    }
}
