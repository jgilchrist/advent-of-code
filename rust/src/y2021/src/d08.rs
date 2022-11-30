use std::collections::HashSet;

use crate::AocSolution;

pub struct Day08;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

struct SignalMapping {
    a: Signal,
    b: Signal,
    c: Signal,
    d: Signal,
    e: Signal,
    f: Signal,
    g: Signal,
}

#[derive(Debug)]
struct SignalMappingCandidates {
    a: HashSet<Signal>,
    b: HashSet<Signal>,
    c: HashSet<Signal>,
    d: HashSet<Signal>,
    e: HashSet<Signal>,
    f: HashSet<Signal>,
    g: HashSet<Signal>,
}

fn initial_candidates() -> HashSet<Signal> {
    let mut hashset = HashSet::new();
    hashset.insert(Signal::A);
    hashset.insert(Signal::B);
    hashset.insert(Signal::C);
    hashset.insert(Signal::D);
    hashset.insert(Signal::E);
    hashset.insert(Signal::F);
    hashset.insert(Signal::G);
    hashset
}

impl SignalMappingCandidates {
    pub fn new() -> SignalMappingCandidates {
        SignalMappingCandidates {
            a: initial_candidates(),
            b: initial_candidates(),
            c: initial_candidates(),
            d: initial_candidates(),
            e: initial_candidates(),
            f: initial_candidates(),
            g: initial_candidates(),
        }
    }

    pub fn process(&mut self, signals: &HashSet<Signal>, candidates: &HashSet<Signal>) {
        self.process_signal(&Signal::A, signals, candidates);
        self.process_signal(&Signal::B, signals, candidates);
        self.process_signal(&Signal::C, signals, candidates);
        self.process_signal(&Signal::D, signals, candidates);
        self.process_signal(&Signal::E, signals, candidates);
        self.process_signal(&Signal::F, signals, candidates);
        self.process_signal(&Signal::G, signals, candidates);
    }

    fn process_signal(
        &mut self,
        signal: &Signal,
        signals: &HashSet<Signal>,
        candidates: &HashSet<Signal>,
    ) {
        if signals.contains(signal) {
            self.narrow(signal, candidates);
        } else {
            self.eliminate(signal, candidates);
        }
    }

    fn narrow(&mut self, signal: &Signal, candidates: &HashSet<Signal>) {
        match signal {
            Signal::A => self.a.retain(|x| candidates.contains(x)),
            Signal::B => self.b.retain(|x| candidates.contains(x)),
            Signal::C => self.c.retain(|x| candidates.contains(x)),
            Signal::D => self.d.retain(|x| candidates.contains(x)),
            Signal::E => self.e.retain(|x| candidates.contains(x)),
            Signal::F => self.f.retain(|x| candidates.contains(x)),
            Signal::G => self.g.retain(|x| candidates.contains(x)),
        };
    }

    fn eliminate(&mut self, signal: &Signal, candidates: &HashSet<Signal>) {
        match signal {
            Signal::A => self.a.retain(|x| !candidates.contains(x)),
            Signal::B => self.b.retain(|x| !candidates.contains(x)),
            Signal::C => self.c.retain(|x| !candidates.contains(x)),
            Signal::D => self.d.retain(|x| !candidates.contains(x)),
            Signal::E => self.e.retain(|x| !candidates.contains(x)),
            Signal::F => self.f.retain(|x| !candidates.contains(x)),
            Signal::G => self.g.retain(|x| !candidates.contains(x)),
        };
    }
}

#[derive(Debug)]
pub struct Definition {
    signal_patterns: Vec<SignalPattern>,
    outputs: Vec<SignalPattern>,
}

#[derive(Debug)]
pub struct SignalPattern(pub Vec<Signal>);

fn parse_signal(signal: char) -> Signal {
    match signal {
        'a' => Signal::A,
        'b' => Signal::B,
        'c' => Signal::C,
        'd' => Signal::D,
        'e' => Signal::E,
        'f' => Signal::G,
        'g' => Signal::G,
        _ => unimplemented!(),
    }
}

fn parse_line(line: &str) -> Definition {
    let components: Vec<&str> = line.split(" | ").collect();
    let (patterns_str, outputs_str) = (components[0], components[1]);

    let signal_patterns = patterns_str
        .split_whitespace()
        .map(|x| SignalPattern(x.chars().map(|c| parse_signal(c)).collect()))
        .collect();

    let outputs = outputs_str
        .split_whitespace()
        .map(|x| SignalPattern(x.chars().map(|c| parse_signal(c)).collect()))
        .collect();

    Definition {
        signal_patterns,
        outputs,
    }
}

impl AocSolution<8> for Day08 {
    type Input = Vec<Definition>;
    type Output = usize;

    fn get_input() -> &'static str {
        include_str!("d08.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input.lines().map(parse_line).collect()
    }

    const PART1_SOLUTION: Option<Self::Output> = None;
    fn part1(i: &Self::Input) -> Self::Output {
        let unambiguous_patterns: Vec<usize> = vec![2, 3, 4, 7];

        let counts: Vec<usize> = i.iter()
            .map(|def| {
                def.outputs
                    .iter()
                    .filter(|&pat| unambiguous_patterns.contains(&pat.0.len()))
                    .count()
            })
            .collect();

        dbg!(&counts);

        counts.iter().sum()
    }

    const PART2_SOLUTION: Option<Self::Output> = None;
    fn part2(i: &Self::Input) -> Self::Output {
        for def in i {
            let mut mapping = SignalMappingCandidates::new();

            fill_mapping_for_1(def, &mut mapping);
            fill_mapping_for_4(def, &mut mapping);
            fill_mapping_for_7(def, &mut mapping);
            fill_mapping_for_8(def, &mut mapping);

            dbg!(mapping);
        }

        todo!();
    }
}

fn fill_mapping_for_1(f: &Definition, mapping: &mut SignalMappingCandidates) {
    let signals_1 = vec![Signal::A, Signal::B];

    let signals = f
        .signal_patterns
        .iter()
        .find(|p| p.0.len() == signals_1.len())
        .unwrap();

    mapping.process(
        &signals_1.iter().cloned().collect(),
        &signals.0.iter().cloned().collect(),
    );
}

fn fill_mapping_for_4(f: &Definition, mapping: &mut SignalMappingCandidates) {
    let signals_4 = vec![Signal::A, Signal::B, Signal::E, Signal::F];

    let signals = f
        .signal_patterns
        .iter()
        .find(|p| p.0.len() == signals_4.len())
        .unwrap();

    mapping.process(
        &signals_4.iter().cloned().collect(),
        &signals.0.iter().cloned().collect(),
    );
}

fn fill_mapping_for_7(f: &Definition, mapping: &mut SignalMappingCandidates) {
    let signals_7 = vec![Signal::A, Signal::B, Signal::D];

    let signals = f
        .signal_patterns
        .iter()
        .find(|p| p.0.len() == signals_7.len())
        .unwrap();

    mapping.process(
        &signals_7.iter().cloned().collect(),
        &signals.0.iter().cloned().collect(),
    );
}

fn fill_mapping_for_8(f: &Definition, mapping: &mut SignalMappingCandidates) {
    let signals_8 = vec![
        Signal::A,
        Signal::B,
        Signal::C,
        Signal::D,
        Signal::E,
        Signal::F,
        Signal::G,
    ];

    let signals = f
        .signal_patterns
        .iter()
        .find(|p| p.0.len() == signals_8.len())
        .unwrap();

    mapping.process(
        &signals_8.iter().cloned().collect(),
        &signals.0.iter().cloned().collect(),
    );
}
