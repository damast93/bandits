trait BanditFunctionality {
    fn draw(&mut self, arm : usize) -> f32;
}

pub struct Bandit {
    functionality : Box<dyn BanditFunctionality>,
    draws_remaining : usize,
    total_payoff : f32,
    num_arms : usize
}

impl Bandit {
    pub fn num_arms(&self) -> usize {
        self.num_arms
    }

    pub fn draws_remaining(&self) -> usize {
        self.draws_remaining
    }

    pub fn payoff(&self) -> f32 {
        self.total_payoff
    }

    pub fn draw(&mut self, arm : usize) -> f32 {
        if self.draws_remaining > 0 {
            self.draws_remaining -= 1;
            let res = self.functionality.draw(arm);
            self.total_payoff += res;
            res
        } else {
            panic!("More draws from a bandit than allowed");
        }
    }
}

pub trait BanditFactory {
    fn initialize(&self, max_draws : usize) -> Bandit;
    fn description(&self) -> String;
}

struct BernoulliBanditFunctionality {
    winning_probabilities : Vec<f32>
}

impl BanditFunctionality for BernoulliBanditFunctionality {
    fn draw(&mut self, arm : usize) -> f32 {
        let p = self.winning_probabilities[arm];
        if rand::random::<f32>() < p { 1.0 } else { 0.0 }
    }
}

pub struct BernoulliFactory {
    pub winning_probabilities : Vec<f32>
}

impl BanditFactory for BernoulliFactory {
    fn initialize(&self, max_draws: usize) -> Bandit {
        Bandit {
            draws_remaining : max_draws,
            num_arms : self.winning_probabilities.len(),
            total_payoff : 0.0,
            functionality : Box::new(BernoulliBanditFunctionality {
                winning_probabilities : self.winning_probabilities.clone()
            })
        }
    }

    fn description(&self) -> String {
        format!("{:?}", self.winning_probabilities).replace(",",";")
    }
}
