// Example2:
//   * Get 'n' Instructions.
//   * Make a CFG.

extern crate radeco;

use radeco::frontend::{esil, r2};
use radeco::middle::{cfg, dot};
use radeco::middle::dot::Dot;

use std::io::prelude::*;
use std::fs::File;

fn make_dot(g: cfg::CFG, outfile: &str) {
    let mut dot_file = File::create(outfile).ok().expect("Error. Cannot create file!\n");
    dot_file.write_all(g.to_dot().as_bytes()).ok().expect("Error. Cannot write file!\n");
    println!("[*] Dot file written!");
    println!("[*] Run `./scripts/genpng.sh {}` to generate the graph.", outfile);
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    // Get a new r2 instance.
    let mut r2 = r2::R2::new("./ex-bins/key");

    // Initialize with sane defaults.
    r2.init();

    // Get 16 Instructions at 'sym.main'
    let mut ops = r2.get_insts(Some(16), Some("sym.main"));
    println!("[*] Got ops.");

    // Initialize the parser with default configurations.
    let mut p = esil::Parser::new(None);
    println!("[*] Begin Parse.");

    for op in ops.iter_mut() {
        p.parse_opinfo(op).ok();
    }

    println!("[*] Begin CFG Generation.");
    let mut cfg = cfg::CFG::new();

    // Extract the instructions out of the parser and construct a CFG.
    cfg.build(&mut (p.emit_insts()));
    println!("[*] Begin Dot generation.");

    // Emit dot for the generated CFG.
    make_dot(cfg, "outputs/ex2.dot");
}
