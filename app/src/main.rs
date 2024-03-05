fn main() {
    let t = true;
    let f = false;
    let unin = [t, f];
    let biin = [(t, t), (t, f), (f, t), (f, f)];
    let unn = ["not"];
    let unop = [not];

    let bin = ["nand", "and", "or", "xor"];
    let biop = [nand, and, or, xor];

    for (op, fname) in unop.iter().zip(unn.into_iter()) {
        println!("Operation: {}", fname);
        for i in unin {
            println!("{} -> {}", i, op(i));
        }
        println!();
    }

    for (op, fname) in biop.iter().zip(bin.into_iter()) {
        println!("Operation: {}", fname);
        for (l, r) in biin {
            println!("({}, {}) -> {}", l, r, op(l, r));
        }
        println!();
    }
}

fn nand(l: bool, r: bool) -> bool {
    !(l & r) // would be implemented in hardware
}

fn not(i: bool) -> bool {
    nand(i, i)
}

fn and(l: bool, r: bool) -> bool {
    not(nand(l, r))
}

fn or(l: bool, r: bool) -> bool {
    nand(not(l), not(r))
}

fn xor(l: bool, r: bool) -> bool {
    and(nand(l, r), or(l, r))
}
