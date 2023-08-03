import random
import numpy as np

class Bandit:
    
    def __init__(self, arms):
        self.arms = list(arms)
        self.num_arms = len(self.arms)
        self.total = 0
        self.draws_remaining = 0
        
    def clone(self):
        c = Bandit(self.arms)
        return c

    def draw(self, arm):
        if self.draws_remaining > 0:
            sampler = self.arms[arm]
            result = sampler()
            self.total += result
            self.draws_remaining -= 1
            return result
        else:
            raise Exception("Bandit has no draws remaining")

# some bandits

def fixed_bandit(ps):
    def fixed(p):
        return lambda: p
    
    arms = [ fixed(p) for p in ps ]
    return Bandit(arms)

def bernoulli_bandit(ps):
    def bernoulli(p):
        return lambda: (1 if random.random() < p else 0)

    arms = [ bernoulli(p) for p in ps ]
    #arms.reverse()
    return Bandit(arms)

########### strategies

def random_fix(bandit):
    n = random.randrange(bandit.num_arms)
    while bandit.draws_remaining > 0:
        bandit.draw(n)        
        
def random_arm(bandit):
    while bandit.draws_remaining > 0:
        bandit.draw(random.randrange(bandit.num_arms))   

def epsilon_first(bandit, epsilon):
    N = bandit.num_arms
    k = int(epsilon * bandit.draws_remaining)
        
    values = np.zeros(N)
    counts = np.zeros(N) + 1.0

    for i in range(k):
        n = random.randrange(N)
        values[n] += bandit.draw(n)
        counts[n] += 1

    n_best = np.argmax(values / counts)
    
    while bandit.draws_remaining > 0:
        bandit.draw(n_best)
        
## write epsilon-greedy


def one_and_done(bandit):
    payoffs = [ bandit.draw(n) for n in range(bandit.num_arms) ]
    n_best = np.argmax(payoffs)
    
    while bandit.draws_remaining > 0:
        bandit.draw(n_best)
        

def thompson(bandit):
    a = [1] * bandit.num_arms
    b = [1] * bandit.num_arms
    
    while bandit.draws_remaining > 0:
        predictions = np.random.beta(a,b)
        n = np.argmax(predictions)
        r = bandit.draw(n)
        if r == 1:
            a[n] += 1
        elif r == 0:
            b[n] += 1
        else:
            raise Exception("Thompson sampler expects 0/1-valued payoff")

####

bandits = [ 
    bernoulli_bandit([0.0, 0.5, 1.0]),
    bernoulli_bandit([0.25, 0.5, 0.75]),
    bernoulli_bandit([0.7, 0.75, 0.8])
  ]

strategies = [ 
    random_arm, 
    random_fix,
    one_and_done,
    lambda b: epsilon_first(b, 0.1),
    lambda b: epsilon_first(b, 0.3),
    thompson
  ]

n_bandits = len(bandits)
n_strategies = len(strategies)

### simulation

N = 1000
N_draws = 100

payoffs = np.zeros((n_bandits, n_strategies))

for (b,bandit) in enumerate(bandits):
    for (s,strategy) in enumerate(strategies):
        payoffs[b,s] = 0
        for episode in range(N):
            c = bandit.clone()
            c.draws_remaining = N_draws
            strategy(c)
            payoffs[b,s] += c.total
            
payoffs /= N

print("strategy,bandit,payoff_mean")
for s in range(n_strategies):
    for b in range(n_bandits):
        print(f"{s}, {b}, {payoffs[b,s]}")