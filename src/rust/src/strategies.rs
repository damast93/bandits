use crate::bandits::Bandit;

mod constant;
mod thompson;
mod epsilongreedy;
mod one_and_done;
mod fixedarm;
mod randomarm;
mod epsilonfirst;

pub trait Strategy {
    fn run(&self, bandit : &mut Bandit); 
    fn name(&self) -> String;
}

pub fn strategies() -> Vec<Box<dyn Strategy>> {
    vec![
        //Box::new(constant::Constant { n : 1 }),
        //Box::new(constant::Constant { n : 2 }),
        Box::new(fixedarm::FixedArm {}),
        Box::new(randomarm::RandomArm {}),
        Box::new(one_and_done::OneAndDone {}),
        Box::new(epsilongreedy::EpsilonGreedy { epsilon: 0.05 }),
        Box::new(epsilongreedy::EpsilonGreedy { epsilon: 0.1 }),
        Box::new(epsilongreedy::EpsilonGreedy { epsilon: 0.2 }),
        Box::new(epsilonfirst::EpsilonFirst { epsilon: 0.05 }),
        Box::new(epsilonfirst::EpsilonFirst { epsilon: 0.1 }),
        Box::new(epsilonfirst::EpsilonFirst { epsilon: 0.2 }),
        Box::new(thompson::Thompson {})

    ]
}