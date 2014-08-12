# compile_msg

[![Build Status](https://travis-ci.org/huonw/compile_msg.png)](https://travis-ci.org/huonw/compile_msg)

A syntax extension for emitting messages at compile time, via the
compiler, similar to `#warning` and `#error` in the C
preprocessor. Four macros are provided (in order of increasing
severity):

- `compile_note`: tell the user a tidbit of information without implying it is a problem,
- `compile_warning`: tell the user that something could go wrong,
- `compile_error`: tell the user about some error, compilation will
  not stop immediately, but will halt before any compiler passes after
  macro expansion.
- `compile_fatal`: tell the user about a catastrophic error and
  immediately halt compilation. `compile_error` is *strongly*
  preferred as it allows further errors and warnings to be picked up
  in a single pass.

The macros can be placed as an item (expanding to nothing), and as an
expression (expanding to a literal unit, i.e. `()`). They are best
used in conditionally compiled items, e.g. if a certain operating
system is entirely unsupported, one can use `compile_error!` with an
appropriate `#[cfg]` attribute.

## Usage

Ensure your `Cargo.toml` contains

```toml
[dependencies.compile_msg]
git = "https://github.com/huonw/compile_msg"
```

and then load the syntax in the normal manner:

```rust
#![feature(phase)]

#[phase(plugin)] extern crate compile_msg;

#[cfg(target_os = "hal")]
compile_error!("I'm sorry, Dave, I'm afraid I can't do that.")


fn main() {
    compile_note!("please be careful"); // note: please be careful


    compile_warning!("take more care"); // warning: take more care


    compile_error!("things are breaking"); // error: things are breaking


    compile_fatal!("catastrophic failure!"); // error: catastrophic failure
    // (compilation stops here)


    compile_warning!("not emitted");
}
```

(If that compiled, it would be equivalent to `fn main() {}` at runtime.)
