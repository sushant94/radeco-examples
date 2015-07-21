extern crate radeco;

use radeco::frontend::{parser, r2};
use radeco::middle::{cfg};
use radeco::middle::dot;
use radeco::middle::ssa::SSAStorage;

use std::env;
use std::io::prelude::*;
use std::fs::File;

fn write_file(fname: &str, res: String) {
    let mut file = File::create(fname).ok().expect("Error. Cannot create file!\n");
    file.write_all(res.as_bytes()).ok().expect("Error. Cannot write file!\n");
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = if args.len() >= 2 { &*args[1] } else { "./ex-bins/simple" };

    // Get a new r2 instance.
    let mut r2 = r2::R2::new(fname);

    // Initialize with sane defaults.
    r2.init();

    // Get Instructions for 'sym.main'
    let func_info = r2.get_function("sym.main").unwrap();

    // Get the ops. We should handle error here. But for this example,
    // Just panic is fine.
    let ops = func_info.ops.unwrap();
    println!("[*] Got ops.");

    // Initialize the parser with default configurations.
    let mut ssa = SSAStorage::new();
    let cfg = {
        // limit scope of p

        let mut p = parser::Parser::new(None);
        println!("[*] Begin Parse.");

        // Get the register profile for the binary an hook it up with the parser.
        let r = r2.get_reg_info().unwrap();
        p.set_register_profile(&r, &mut ssa);
        p.run(ops, func_info.addr.unwrap()).unwrap();

        // for op in ops.iter_mut() {
        //     p.parse_opinfo(op).ok();
        // }

        println!("[*] Begin CFG Generation.");
        let mut cfg = cfg::CFG::new();
        cfg.build(&mut (p.emit_insts()));
        cfg
    };

    println!("[*] Begin Dot generation.");
    let res_cfg = dot::emit_dot(&cfg);
    let res_ssa = dot::emit_dot(&ssa);

    let outfile = "outputs/ex5-cfg.dot";
    write_file(outfile, res_cfg);
    println!("[*] Run `./scripts/genpng.sh {}` to generate the cf-graph.", outfile);

    let outfile = "outputs/ex5-ssa.dot";
    write_file(outfile, res_ssa);
    println!("[*] Run `./scripts/genpng.sh {}` to generate the ssa-graph.", outfile);
}
