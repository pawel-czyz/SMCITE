use rand_distr::{Distribution, Uniform};

pub type LogProb = f32;

pub fn metropolis_ratio(
    logp1: LogProb,
    logp2: LogProb,
    log1given2: LogProb,
    log2given1: LogProb,
) -> LogProb {
    let a = logp2 + log1given2 - (logp1 + log2given1);
    return LogProb::min(1.0, a.exp());
}

/// Performs a Metropolis–Hastings step.
pub fn metropolis_hastings_step<S>(
    rng: &mut impl rand::Rng,
    state1: S,
    state2: S,
    logp1: LogProb,
    logp2: LogProb,
    log1given2: LogProb,
    log2given1: LogProb,
) -> S {
    let a = metropolis_ratio(logp1, logp2, log1given2, log2given1);

    let uniform = Uniform::<LogProb>::new(0.0, 1.0);
    let u: LogProb = uniform.sample(rng);

    if u < a {
        return state2;
    } else {
        return state1;
    }
}

/// Performs a Metropolis step, i.e., assumes that `q(1|2) = q(2|1)`.
pub fn metropolis_symmetric_step<S>(
    rng: &mut impl rand::Rng,
    state1: S,
    state2: S,
    logp1: LogProb,
    logp2: LogProb,
) -> S {
    return metropolis_hastings_step(rng, state1, state2, logp1, logp2, 0.0, 0.0);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(1 + 8, 3);
    }
}
