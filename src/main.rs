type Label = &'static str;
type Loc = &'static str;

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

fn print_states(process: Vec<Trans>) {}

fn print_process(process: Vec<Trans>) {
    print!("digraph {{");
    print_states(process);
    println!("");
}

fn main() {
    let p01: fn(SharedVars) -> SharedVars = |sv| SharedVars { t1: sv.x, ..sv };
    let p12: fn(SharedVars) -> SharedVars = |sv| SharedVars {
        t1: sv.t1 + 1,
        ..sv
    };
    let p23: fn(SharedVars) -> SharedVars = |sv| SharedVars { x: sv.t1, ..sv };
    let tt: fn(SharedVars) -> bool = trans_true;
    let process_P = vec![
        Trans::new("P0", "read", "P1", tt, p01),
        Trans::new("P1", "inc", "P2", tt, p12),
        Trans::new("P2", "write", "P3", tt, p23),
    ];
    print_process(process_P);
}
