type Label = &'static str;
type Loc = &'static str;

struct SharedVars {
    x: i32,
    t1: i32,
    t2: i32,
}

#[derive(Debug, Clone)]
struct Trans {
    label: Label,
    loc: Loc,
}

impl Trans {
    fn new(label: Label, loc: Loc) -> Self {
        Trans { label, loc }
    }
}

fn trans_true(_sv: SharedVars) -> bool {
    true
}

fn main() {
    let trans1 = Trans::new("read", "loc");
    let process = vec![trans1];
    let p01: fn(SharedVars) -> SharedVars = |sv| SharedVars { t1: sv.x, ..sv };
    let p12: fn(SharedVars) -> SharedVars = |sv| SharedVars {
        t1: sv.t1 + 1,
        ..sv
    };
    let p23: fn(SharedVars) -> SharedVars = |sv| SharedVars { x: sv.t1, ..sv };
    let process_P = [
        ("P0", ("read", "P1", trans_true, p01)),
        ("P1", ("inc", "P2", trans_true, p12)),
        ("P2", ("write", "P3", trans_true, p23)),
    ];
}
