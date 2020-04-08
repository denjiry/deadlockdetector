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
    let pp01: fn(SharedVars) -> SharedVars = p01;
    // let pp01: fn(SharedVars) -> SharedVars = |sv| SharedVars {
    //     x: sv.x,
    //     t1: sv.x,
    //     t2: sv.t2,
    // };
    let pp12: fn(SharedVars) -> SharedVars = p12;
    let pp23: fn(SharedVars) -> SharedVars = p23;
    let process_P = [
        ("P0", ("read", "P1", trans_true, pp01)),
        ("P1", ("inc", "P2", trans_true, pp12)),
        ("P2", ("write", "P3", trans_true, pp23)),
    ];
}

fn p01(sv: SharedVars) -> SharedVars {
    SharedVars {
        x: sv.x,
        t1: sv.x,
        t2: sv.t2,
    }
}

fn p12(sv: SharedVars) -> SharedVars {
    SharedVars {
        x: sv.t1,
        t1: sv.t1,
        t2: sv.t2,
    }
}

fn p23(sv: SharedVars) -> SharedVars {
    SharedVars {
        x: sv.x,
        t1: sv.x,
        t2: sv.t2,
    }
}
