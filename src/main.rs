use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, btree_map},
    fmt::Write,
};

use clap::Parser;
use lender::Lender;

mod ordered_pairings;
mod partition_parts;

use ordered_pairings::OrderedPairings;
use partition_parts::PartitionsParts;

/// If n_unique_descriptors < n_unique_digits, then
/// (n_unique_digits - n_unique_descriptors) values from reps must appear
/// in descriptors.
/// If there are n_ones 1s in the descriptors, then
/// n_unique_descriptors + n_ones - 1 must be less than or equal to n_unique_digits.
fn check_free_vars(reps: &[u8], descriptors: &[u8]) -> bool {
    let n_unique_digits = descriptors.len();
    let n_unique_descriptors = HashSet::<&u8>::from_iter(descriptors).len();
    let n_ones = descriptors.iter().take_while(|d| **d == 1).count();
    if n_ones != 0 && n_unique_descriptors + n_ones - 1 > n_unique_digits {
        false
    } else if n_unique_descriptors == n_unique_digits {
        true
    } else {
        let mut reps_required_in_descriptors =
            n_unique_digits - n_unique_descriptors;
        let mut reps = reps.iter();
        while reps_required_in_descriptors > 0 {
            let Some(rep) = reps.next() else { return false };
            if descriptors.iter().any(|desc| rep == desc) {
                reps_required_in_descriptors -= 1;
            }
        }
        true
    }
}

/// Each descriptor's reps must be less than or equal to the descriptor.
/// For the descriptor 9, the greatest possible number of reps is 7.
fn check_reps_lte_descriptor(rep_descriptors: &[(u8, u8)]) -> bool {
    rep_descriptors.iter().all(
        |(reps, descriptor)| reps <= descriptor, /* && (*descriptor != 9 || *reps < 7) */
    )
}

/// If a rep is equal to a descriptor, then we need a free var
fn check_free_vars_rep_descriptor(rep_descriptors: &[(u8, u8)]) -> bool {
    let n_unique_digits = rep_descriptors.len();
    let mut n_unique_descriptors = 0usize;
    // i'th element indicates if the descriptor i+1 exists in descriptors
    let mut descriptor_used = [false; 9];
    let mut free_vars_needed = 0usize;
    for (rep, d) in rep_descriptors {
        if !std::mem::replace(&mut descriptor_used[(d - 1) as usize], true) {
            n_unique_descriptors += 1;
        }
        if rep == d {
            free_vars_needed += 1;
        }
    }
    let free_vars = n_unique_digits - n_unique_descriptors;
    free_vars == free_vars_needed
}

enum Described {
    Digit(u8),
    /// Index of the var set
    Var(u8),
}

struct Solution {
    best_solution: Vec<(u8, u8, Described)>,
    vars: BTreeMap<u8, BTreeSet<u8>>,
}

impl Solution {
    fn new(rep_descriptors: &[(u8, u8)]) -> Self {
        let mut unique_descriptor_counts = BTreeMap::new();
        // possible slots for a digit that occurs k times in descriptors
        let mut slots_to_rep_descriptors =
            HashMap::<u8, Vec<_>>::with_capacity(rep_descriptors.len());
        let mut descriptor_to_reps = BTreeMap::<u8, Vec<u8>>::new();
        for (reps, descriptor) in rep_descriptors {
            unique_descriptor_counts
                .entry(*descriptor)
                .and_modify(|count| *count += *reps)
                .or_insert(*reps);
            slots_to_rep_descriptors
                .entry(*descriptor - *reps)
                .or_default()
                .push((reps, descriptor));
            descriptor_to_reps
                .entry(*descriptor)
                .or_default()
                .push(*reps);
        }
        for rep_descriptors in slots_to_rep_descriptors.values_mut() {
            rep_descriptors
                .sort_by_key(|(reps, descriptor)| (*descriptor, *reps));
        }
        for reps in descriptor_to_reps.values_mut() {
            reps.sort();
        }
        let n_unique_descriptors = unique_descriptor_counts.len();
        let n_free_vars = rep_descriptors.len() - n_unique_descriptors;
        let mut slots_to_vars = BTreeMap::<u8, BTreeSet<u8>>::new();
        if n_free_vars != 0 {
            let possible_free_vars: BTreeSet<u8> = (0..9)
                .filter(|d| !unique_descriptor_counts.contains_key(d))
                .collect();
            slots_to_vars.insert(0, possible_free_vars);
        }
        for (descriptor, count) in unique_descriptor_counts {
            slots_to_vars.entry(count).or_default().insert(descriptor);
        }
        let mut res = Self {
            best_solution: Vec::with_capacity(rep_descriptors.len()),
            vars: {
                let mut vars = slots_to_vars.clone();
                vars.retain(|_, vs| vs.len() > 1);
                vars
            },
        };
        for (descriptor, reps) in descriptor_to_reps.into_iter().rev() {
            let mut reps_digits = Vec::with_capacity(reps.len());
            for rep in reps {
                let diff = descriptor - rep;
                let (described, best_digit) = match slots_to_vars.entry(diff) {
                    btree_map::Entry::Occupied(mut digit_set) => {
                        if !res.vars.contains_key(&diff) {
                            let mut digit_set = digit_set.remove();
                            let digit = digit_set.pop_last().unwrap();
                            (Described::Digit(digit), digit)
                        } else {
                            let best_digit =
                                digit_set.get_mut().pop_last().unwrap();
                            (Described::Var(diff), best_digit)
                        }
                    }
                    btree_map::Entry::Vacant(_) => {
                        unreachable!()
                    }
                };
                reps_digits.push((rep, best_digit, described));
            }
            reps_digits.sort_by_key(|(rep, best_digit, _)| (*best_digit, *rep));
            for (rep, _, described) in reps_digits.into_iter().rev() {
                res.best_solution.push((rep, descriptor, described));
            }
        }
        res
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "reps:        ".fmt(f)?;
        for (rep, _desc, _digit) in &self.best_solution {
            rep.fmt(f)?;
        }
        "\ndescriptors: ".fmt(f)?;
        for (_rep, desc, _digit) in &self.best_solution {
            desc.fmt(f)?;
        }
        "\ndigits:      ".fmt(f)?;
        // Pairs of idents and vars
        let mut idents_vars: BTreeMap<u8, _> = self
            .vars
            .iter()
            .map(|(idx, vars)| (*idx, (BTreeSet::<char>::new(), vars)))
            .collect();
        let mut new_ident = {
            const LOWERCASE_A_ASCII: u8 = 0x61;
            let mut count = 0;
            move || {
                let ident = if count < 8 {
                    char::from(LOWERCASE_A_ASCII + count)
                } else {
                    // Avoid using `i` as an ident
                    char::from(LOWERCASE_A_ASCII + count + 1)
                };
                count += 1;
                ident
            }
        };
        for (_rep, _desc, digit) in &self.best_solution {
            match digit {
                Described::Digit(d) => d.fmt(f)?,
                Described::Var(v) => {
                    let ident = new_ident();
                    idents_vars.get_mut(v).unwrap().0.insert(ident);
                    ident.fmt(f)?;
                }
            }
        }
        f.write_char('\n')?;
        if !idents_vars.is_empty() {
            "where\n".fmt(f)?;
            for (idents, vars) in idents_vars.into_values().rev() {
                "  {".fmt(f)?;
                let idents_len = idents.len();
                for (idx, ident) in idents.into_iter().enumerate() {
                    ident.fmt(f)?;
                    if idx < idents_len - 1 {
                        ", ".fmt(f)?
                    }
                }
                "} ⊆ {".fmt(f)?;
                let vars_len = vars.len();
                for (idx, var) in vars.iter().enumerate() {
                    var.fmt(f)?;
                    if idx < vars_len - 1 {
                        ", ".fmt(f)?
                    }
                }
                "}\n".fmt(f)?
            }
        }
        Ok(())
    }
}

/// Check that for each unique descriptor, a rep exists such that the
/// descriptor count is correct
fn check_reps_descriptor_counts(
    rep_descriptors: &[(u8, u8)],
) -> Option<Solution> {
    let mut unique_descriptor_counts = BTreeMap::new();
    // possible slots for a digit that occurs k times in descriptors
    let mut slot_counts = HashMap::with_capacity(rep_descriptors.len());
    for (rep, descriptor) in rep_descriptors {
        unique_descriptor_counts
            .entry(*descriptor)
            .and_modify(|count| *count += *rep)
            .or_insert(*rep);
        slot_counts
            .entry(*descriptor - *rep)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut slots_needed = HashMap::with_capacity(rep_descriptors.len());
    for unique_descriptor_count in unique_descriptor_counts.values() {
        slots_needed
            .entry(unique_descriptor_count)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    for (k, slots_needed) in slots_needed {
        if let Some(slots_available) = slot_counts.get(k)
            && *slots_available >= slots_needed
        {
            continue;
        } else {
            return None;
        }
    }
    Some(Solution::new(rep_descriptors))
}

fn solve(n_pairs: usize) {
    for n_unique_digits in 1..=n_pairs.min(10) {
        println!("{n_unique_digits} UNIQUE DIGITS:");
        for reps in PartitionsParts::new(n_unique_digits, n_pairs) {
            'descrs: for descriptors in
                PartitionsParts::new(n_unique_digits, n_pairs * 2)
            {
                if reps > descriptors {
                    continue 'descrs;
                }
                if !check_free_vars(&reps, &descriptors) {
                    continue 'descrs;
                }
                let mut rep_descriptors =
                    OrderedPairings::new(&reps, &descriptors);
                'rep_descriptors: while let Some(rep_descriptors) =
                    rep_descriptors.next()
                {
                    if !check_reps_lte_descriptor(rep_descriptors) {
                        continue 'rep_descriptors;
                    }
                    if !check_free_vars_rep_descriptor(rep_descriptors) {
                        continue 'rep_descriptors;
                    }
                    let Some(solution) =
                        check_reps_descriptor_counts(rep_descriptors)
                    else {
                        continue 'rep_descriptors;
                    };
                    println!("{solution}");
                }
            }
        }
    }
}

#[derive(Parser)]
struct Cli {
    pairs: usize,
}

fn main() {
    let cli = Cli::parse();
    solve(cli.pairs);
}
