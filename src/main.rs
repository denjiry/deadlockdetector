use std::collections::HashMap;

type Label = &'static str;
type Loc = &'static str;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct SharedVars {
    x: i32,
    t1: i32,
    t2: i32,
}

#[derive(Clone, Copy)]
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

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    sv: SharedVars,
    locs: Vec<Loc>,
}

type Process = Vec<Trans>;
type Path = Vec<(Label, State)>;
fn concurrent_composition(r0: SharedVars, ps: Vec<Process>) -> Vec<Path> {
    let s0 = State {
        sv: r0,
        locs: ps.clone().iter().map(|p| p[0].source).collect(),
    };
    let label0 = "---";
    let mut htable = HashMap::new();
    htable.insert(s0.clone(), (0, vec![]));
    let que: Vec<(State, u8, Path)> = vec![(s0.clone(), 0, vec![(label0, s0)])];
    let mut deadlocks = Vec::new();
    for (state, id, path) in que {
        let transes = collect_trans(&state, &ps);
        if transes.is_empty() {
            deadlocks.push(path);
        }
        htable.insert(state, (id, transes));
    }
    deadlocks
}

fn collect_trans(state: &State, ps: &Vec<Process>) -> Vec<(Label, State)> {
    let mut lts = Vec::new();
    let sv = state.sv;
    let locs = &state.locs;
    assert_eq!(locs.len(), ps.len());
    for (i, process) in ps.iter().enumerate() {
        for trans in process.iter() {
            let guard = trans.guard;
            let action = trans.action;
            let label = trans.label;
            if guard(sv) {
                let mut new_locs = locs.clone();
                new_locs[i] = trans.dest;
                let new_state = State {
                    sv: action(sv),
                    locs: new_locs,
                };
                lts.push((label, new_state));
            }
        }
    }
    lts
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
