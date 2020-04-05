type Label = &'static str;
type Loc = &'static str;

struct SharedVars {
    x: i32,
    t1: i32,
    t2: i32,
}

#[derive(Debug)]
struct Process {
    label: Label,
    loc: Loc,
}

impl Process {
    fn new(label: Label, loc: Loc) -> Self {
        Process { label, loc }
    }
}

fn main() {
    let process = Process::new("label", "loc");
    println!("{:?}", process);
}
