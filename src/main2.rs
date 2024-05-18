// use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_distr::{Distribution, Normal, Uniform};

type LogProb = f32;
type State = f64;


fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);

    let mut state: State = -8.0;
    // let proposal = Uniform::<f64>::new(0.0, 1.0);
    let proposal = Normal::new(0.5, 0.4).unwrap();

    println!("Before start: {}", state);

    let n_samples = 10000;

    for _step in 0..n_samples {
        let candidate = state - 0.5 + proposal.sample(&mut rng);

        let logp_state = log_density(state);
        let logp_candidate = log_density(candidate);

        state = metropolis_symmetric_step(&mut rng, state, candidate, logp_state, logp_candidate);
    }
    println!("After warm-up: {}", state);

    let mut total_sum: State = 0.0;

    for _step in 0..n_samples {
        let candidate = state - 0.5 + proposal.sample(&mut rng);

        let logp_state = log_density(state);
        let logp_candidate = log_density(candidate);

        state = metropolis_symmetric_step(&mut rng, state, candidate, logp_state, logp_candidate);
        total_sum = total_sum + state;
    }

    println!("Mean: {}", total_sum / State::from(n_samples));
}

fn log_density(x: State) -> LogProb {
    return -(x - 1.0).powi(2) as LogProb;
}

fn _metropolis_ratio(logp1: LogProb, logp2: LogProb, log1given2: LogProb, log2given1: LogProb) -> LogProb {
    let a = logp2 + log1given2 - (logp1 + log2given1);
    return LogProb::min(1.0, a.exp());
}

fn metropolis_hastings_step<S>(
    rng: &mut impl rand::Rng,
    state1: S,
    state2: S,
    logp1: LogProb,
    logp2: LogProb,
    log1given2: LogProb,
    log2given1: LogProb,
) -> S {
    let a = _metropolis_ratio(logp1, logp2, log1given2, log2given1);

    let uniform = Uniform::<LogProb>::new(0.0, 1.0);
    let u: LogProb = uniform.sample(rng);

    if u < a {
        return state2;
    } else {
        return state1;
    }
}

fn metropolis_symmetric_step<S>(
    rng: &mut impl rand::Rng,
    state1: S,
    state2: S,
    logp1: LogProb,
    logp2: LogProb,
) -> S {
    return metropolis_hastings_step(rng, state1, state2, logp1, logp2, 0.0, 0.0);
}
