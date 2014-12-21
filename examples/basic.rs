#![feature(phase)]

#[phase(plugin)] extern crate compile_msg;

#[cfg(rare)]
compile_note!("only emitted with --cfg rate");

fn main() {
    compile_note!("useful information: ", 1, " instance");

    compile_warning!("x");

    compile_error!("y");

    // compilation stops here
    compile_fatal!("z");

    compile_note!("not emitted")
}
