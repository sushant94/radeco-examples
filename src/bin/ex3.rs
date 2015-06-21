// Example3:
//   * Get instructions for a complete function.
//   * Attach register and flag information.
//   * Make a CFG.

extern crate radeco;

use radeco::frontend::{parser, r2};
use radeco::middle::{cfg};
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

    // Get Instructions for 'sym.main'
    let func_info = r2.get_function("sym.main");

    // Get the ops. We should handle error here. But for this example,
    // Just panic is fine.
    let mut ops = func_info.ops.unwrap();
    println!("[*] Got ops.");

    // Initialize the parser with default configurations.
    let mut p = parser::Parser::new(None);
    println!("[*] Begin Parse.");
    
    // Get the register profile for the binary an hook it up with the parser.
    let r = r2.get_reg_info();
    p.set_register_profile(&r);
    let flags = r2.get_flag_info();
    p.set_flags(&flags);

    for op in ops.iter_mut() {
        p.parse_opinfo(op).ok();
    }

    println!("[*] Begin CFG Generation.");
    let mut cfg = cfg::CFG::new();

    // Extract the instructions out of the parser and construct a CFG.
    cfg.build(&mut (p.emit_insts()));
    println!("[*] Begin Dot generation.");

    // Emit dot for the generated CFG.
    make_dot(cfg, "outputs/ex3.dot");
}
