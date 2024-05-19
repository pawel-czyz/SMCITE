[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip) [![Rust](https://github.com/pawel-czyz/SMCITE/actions/workflows/rust.yml/badge.svg)](https://github.com/pawel-czyz/SMCITE/actions/workflows/rust.yml)

# BlackForest

**Note:** This is an early-stage project, which has not reached any stable release yet. Frequent changes to the API (and possibly bugs) are expected.  

## What are we aiming for?

**tl;dr:** [BlackJAX](https://github.com/blackjax-devs/blackjax) for trees modeling cancer evolution.

**Background:** There exist many models aiming at the reconstruction of a phylogenetic tree from observational data sequenced at the single-cell level (e.g., [SCITE](https://github.com/cbg-ethz/SCITE), [SCICoNE](https://github.com/cbg-ethz/SCICoNE), [COMPASS](https://github.com/cbg-ethz/COMPASS), [PYggdrasil](https://github.com/cbg-ethz/pyggdrasil)).

They usually consist of proposing a likelihood function, connecting the tree to the observed data, and a method for optimizing the likelihood or sampling it using Markov chain Monte Carlo algorithms.

This package aims at providing an ecosystem such that:

  - It allows easy development of new models, thanks to orthogonal abstractions built around sampling on tree spaces.
  - Does not only include Markov chain Monte Carlo samplers, but als Sequential Monte Carlo samplers.
  - Is fast and supports multithreading.
  - Although the main sampling part is implemented in Rust, it integrates well with Python and Snakemake thanks to [Maturin](https://www.maturin.rs/).


## Disclaimer
This is an early-stage experimental project and it may be later abandoned due to the lack of time, computational issue which have not been anticipated, or unforeseen reasons.
We welcome contributions, but at the same time we suggest to not rely on this package at this stage. 

## Why a new package?

There are several alternatives we have considered:
  - [BlackJAX](https://github.com/blackjax-devs/blackjax) would be our preferred choice due to a great ecosystem of different samplers. However, we found working with combinatorial structures, such as trees with variable number of nodes, not very convenient in JAX (which is however excellent for working with array-like objects). 
  - [Blang](https://www.stat.ubc.ca/~bouchard/blang/), which is a full probabilistic programming language, but does not seem to support models for cancer phylogeny inference.
  - Julia has an excellent ecosystem consisting of [Pigeons.jl](https://github.com/Julia-Tempering/Pigeons.jl) and [Turing.jl](https://turing.ml/). However, Rust has better capabilities for integration with Python and static compilation.

