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
    let process = Process::new("label", "loc");
    println!("{:?}", process);
}
