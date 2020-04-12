use std::collections::HashMap;

type Label = &'static str;
type Loc = &'static str;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SharedVars {
    x: i32,
    t1: i32,
    t2: i32,
}

#[derive(Clone)]
struct Trans {
    source: Loc,
    label: Label,
    dest: Loc,
    guard: fn(SharedVars) -> bool,
    action: fn(SharedVars) -> SharedVars,
}

impl Trans {
    fn new(
        source: Loc,
        label: Label,
        dest: Loc,
        guard: fn(SharedVars) -> bool,
        action: fn(SharedVars) -> SharedVars,
    ) -> Self {
        Trans {
            source,
            label,
            dest,
            guard,
            action,
        }
    }
}

fn trans_true(_sv: SharedVars) -> bool {
    true
}

fn print_states(process: Vec<Trans>) {
    for p in process.iter() {
        println!("{:?};", p.source);
    }
}

fn print_trans(process: Vec<Trans>) {
    for p in process.iter() {
        println!("{:?} -> {:?} [label={:?}];", p.source, p.dest, p.label);
    }
}

fn print_process(process: &Vec<Trans>) {
    print!("digraph {{");
    print_states(process.to_vec());
    print_trans(process.to_vec());
    println!("}}");
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    sv: SharedVars,
    locs: Vec<Loc>,
}

type Process = Vec<Trans>;
fn concurrent_composition(r0: SharedVars, ps: Vec<Process>) {
    let s0 = State {
        sv: r0,
        locs: ps.iter().map(|p| p[0].source).collect(),
    };
    let label0 = "---";
    let mut htable = HashMap::new();
    htable.insert(&s0, (0, vec![]));
    let que = vec![(&s0, 0, vec![(label0, &s0)])];
    let deadlocks = Vec::<u8>::new();
    loop {
        let (state, id, path) = que.pop().unwrap();
        let transes = collect_trans(state, ps);
    }
}

fn collect_trans(state: State, ps: Vec<Process>) -> Vec<(Label, Loc)> {
    let lts = Vec::new();
    let locs: Vec<Loc> = state.locs;
    for (loc, process) in Vec::zip(locs.iter(), ps.iter()).iter() {
        for trans in process.iter() {
            if trans.guard(state.sv) {
                let sv = trans.action(state.sv);
            }
        }
    }
}

fn main() {
    let p01: fn(SharedVars) -> SharedVars = |sv| SharedVars { t1: sv.x, ..sv };
    let p12: fn(SharedVars) -> SharedVars = |sv| SharedVars {
        t1: sv.t1 + 1,
        ..sv
    };
    let p23: fn(SharedVars) -> SharedVars = |sv| SharedVars { x: sv.t1, ..sv };
    let tt: fn(SharedVars) -> bool = trans_true;
    let process_p = vec![
        Trans::new("P0", "read", "P1", tt, p01),
        Trans::new("P1", "inc", "P2", tt, p12),
        Trans::new("P2", "write", "P3", tt, p23),
    ];
    print_process(&process_p);

    let r0 = SharedVars { x: 0, t1: 0, t2: 0 };
    let ps = vec![process_p];
    concurrent_composition(r0, ps);
}
