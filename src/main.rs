use std::collections::{HashMap, VecDeque};

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
    target: Loc,
    guard: fn(SharedVars) -> bool,
    action: fn(SharedVars) -> SharedVars,
}

impl Trans {
    fn new(
        source: Loc,
        label: Label,
        target: Loc,
        guard: fn(SharedVars) -> bool,
        action: fn(SharedVars) -> SharedVars,
    ) -> Self {
        Trans {
            source,
            label,
            target,
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
        println!("{:?} -> {:?} [label={:?}];", p.source, p.target, p.label);
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

#[derive(Hash, PartialEq, Eq, Clone)]
struct Node {
    label: Label,
    state: State,
}

type Process = Vec<Trans>;
type Path = Vec<Node>;
fn concurrent_composition(r0: SharedVars, ps: Vec<Process>) -> Vec<Path> {
    let s0 = State {
        sv: r0,
        locs: ps.clone().iter().map(|p| p[0].source).collect(),
    };
    let label0 = "---";
    let mut htable: HashMap<State, (usize, Path)> = HashMap::new();
    htable.insert(s0.clone(), (0, vec![]));
    let path0 = vec![Node {
        label: label0,
        state: s0.clone(),
    }];
    let mut que: VecDeque<(State, usize, Path)> = VecDeque::new();
    que.push_back((s0.clone(), 0, path0));
    let mut deadlocks = Vec::new();
    while !que.is_empty() {
        let (state, id, path) = que.pop_front().unwrap();
        let transes: Path = collect_trans(&state, &ps);
        if transes.is_empty() {
            deadlocks.push(path.clone());
        }
        htable.insert(state.clone(), (id, transes.clone()));
        for node in transes {
            if !htable.contains_key(&node.state) {
                let id = htable.len();
                htable.insert(node.state.clone(), (id, vec![]));
                // Queue.add (target, id, (label, target)::path) que)
                let mut new_path = vec![node.clone()];
                new_path.append(&mut path.clone());
                que.push_back((node.state, id, new_path));
            }
        }
    }
    deadlocks
}

fn collect_trans(st: &State, ps: &Vec<Process>) -> Vec<Node> {
    let mut lts = Vec::new();
    let sv = st.sv;
    let locs = &st.locs;
    assert_eq!(locs.len(), ps.len());
    for (i, process) in ps.iter().enumerate() {
        for trans in process.iter() {
            let guard = trans.guard;
            let action = trans.action;
            let label = trans.label;
            if guard(sv) {
                let mut new_locs = locs.clone();
                new_locs[i] = trans.target;
                let state = State {
                    sv: action(sv),
                    locs: new_locs,
                };
                lts.push(Node { label, state });
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
