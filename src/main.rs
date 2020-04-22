use std::collections::{HashMap, VecDeque};

type Label = &'static str;
type Loc = &'static str;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct SharedVars {
    x: i32,
    t1: i32,
    t2: i32,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct State {
    sv: SharedVars,
    locs: Vec<Loc>,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Node {
    label: Label,
    state: State,
}

type Process = Vec<Trans>;
type Path = Vec<Node>;
fn concurrent_composition(
    r0: SharedVars,
    ps: Vec<Process>,
) -> (HashMap<State, (usize, Path)>, Vec<Path>) {
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
        let (state, id, path) = que.pop_front().expect("que must not be empty.");
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
    (htable, deadlocks)
}

fn collect_trans(st: &State, ps: &Vec<Process>) -> Vec<Node> {
    let mut lts = Vec::new();
    let sv = st.sv;
    let locs = &st.locs;
    assert_eq!(locs.len(), ps.len());
    for (i, process) in ps.iter().enumerate() {
        for trans in process.iter().filter(|trans| trans.source == locs[i]) {
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

fn print_deadlocks(deadlocks: Vec<Path>) {
    println!("print_deadlocks");
    for (i, deadlock) in deadlocks.iter().enumerate() {
        println!("Deadlock: {:>2}", i);
        for node in deadlock.iter().rev() {
            println!(
                "label:{:>6}  {:?} {:?} ",
                node.label, node.state.sv, node.state.locs
            );
        }
        println!("");
    }
}

fn viz_lts(htable: HashMap<State, (usize, Path)>) {
    println!("viz_lts");
    println!("digraph {{");
    // print state
    for (state, (id, path)) in &htable {
        print!("{} [label=\"{} \\n", &id, &id);
        // let locs = state.locs;
        for loc in &state.locs {
            print!("{} ", loc);
        }
        let sv = state.sv;
        print!("\\n x={} t1={} t2={}\",", sv.x, sv.t1, sv.t2);
        print!(
            "{}",
            if *id == 0 {
                "style=filled,fillcolor=cyan"
            } else if path.is_empty() {
                "style=filled,fillcolor=pink"
            } else {
                ""
            }
        );
        println!("];")
    }
    // print trans
    for (_state, (sid, path)) in &htable {
        for node in path {
            let (tid, _) = htable
                .get(&node.state)
                .expect("the key must exists because it is its own key");
            println!("{} -> {} [label=\"{}\"];", sid, tid, node.label);
        }
    }
    println!("}}");
}

fn main() {
    let p01: fn(SharedVars) -> SharedVars = |sv| SharedVars { t1: sv.x, ..sv };
    let p12: fn(SharedVars) -> SharedVars = |sv| SharedVars {
        t1: sv.t1 + 1,
        ..sv
    };
    let p23: fn(SharedVars) -> SharedVars = |sv| SharedVars { x: sv.t1, ..sv };
    let q01: fn(SharedVars) -> SharedVars = |sv| SharedVars { t2: sv.x, ..sv };
    let q12: fn(SharedVars) -> SharedVars = |sv| SharedVars {
        t2: sv.t2 + 1,
        ..sv
    };
    let q23: fn(SharedVars) -> SharedVars = |sv| SharedVars { x: sv.t2, ..sv };
    let tt: fn(SharedVars) -> bool = trans_true;
    let process_p = vec![
        Trans::new("P0", "read", "P1", tt, p01),
        Trans::new("P1", "inc", "P2", tt, p12),
        Trans::new("P2", "write", "P3", tt, p23),
    ];
    let process_q = vec![
        Trans::new("Q0", "read", "Q1", tt, q01),
        Trans::new("Q1", "inc", "Q2", tt, q12),
        Trans::new("Q2", "write", "Q3", tt, q23),
    ];
    print_process(&process_p);
    print_process(&process_q);

    let r0 = SharedVars { x: 0, t1: 0, t2: 0 };
    let ps = vec![process_p, process_q];
    let (htable, deadlocks) = concurrent_composition(r0, ps);
    print_deadlocks(deadlocks);
    viz_lts(htable);
}
