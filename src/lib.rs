#![feature(plugin_registrar)]
#![crate_type = "dylib"]
#![allow(unstable)]

extern crate syntax;
extern crate rustc;

use syntax::ast;
use syntax::codemap::{Span};
use syntax::ext::base;
use syntax::ext::base::{ExtCtxt, DummyResult};
use rustc::plugin::Registry;

use Severity::{Note, Warning, Error, Fatal};

enum Severity {
    Note,
    Warning,
    Error,
    Fatal,
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    macro_rules! register_specifics {
        ($($name: expr => $sev: ident),*) => {
            {
                $(
                    reg.register_macro($name, {
                        fn f(cx: &mut ExtCtxt, sp: Span,
                             args: &[ast::TokenTree]) -> Box<base::MacResult+'static> {
                            expand_msg($sev, cx, sp, args)
                        }
                        f
                    });
                    )*
            }
        }
    }

    register_specifics! {
        "compile_note" => Note,
        "compile_warning" => Warning,
        "compile_error" => Error,
        "compile_fatal" => Fatal
    }
}

fn expand_msg(sev: Severity,
              cx: &mut ExtCtxt, sp: Span, args: &[ast::TokenTree]) -> Box<base::MacResult+'static> {
    // copied from syntax::ext::concat.
    let es = match base::get_exprs_from_tts(cx, sp, args) {
        Some(e) => e,
        None => return base::DummyResult::expr(sp)
    };
    let mut accumulator = String::new();
    for e in es.into_iter() {
        match e.node {
            ast::ExprLit(ref lit) => {
                match lit.node {
                    ast::LitStr(ref s, _) |
                    ast::LitFloat(ref s, _) |
                    ast::LitFloatUnsuffixed(ref s) => {
                        accumulator.push_str(s.get());
                    }
                    ast::LitChar(c) => {
                        accumulator.push(c);
                    }
                    ast::LitInt(i, ast::UnsignedIntLit(_)) |
                    ast::LitInt(i, ast::SignedIntLit(_, ast::Plus)) |
                    ast::LitInt(i, ast::UnsuffixedIntLit(ast::Plus)) => {
                        accumulator.push_str(format!("{}", i).as_slice());
                    }
                    ast::LitInt(i, ast::SignedIntLit(_, ast::Minus)) |
                    ast::LitInt(i, ast::UnsuffixedIntLit(ast::Minus)) => {
                        accumulator.push_str(format!("-{}", i).as_slice());
                    }
                    ast::LitBool(b) => {
                        accumulator.push_str(format!("{}", b).as_slice());
                    }
                    ast::LitByte(..) |
                    ast::LitBinary(..) => {
                        cx.span_err(e.span, "cannot concatenate a binary literal");
                    }
                }
            }
            _ => {
                cx.span_err(e.span, "expected a literal");
            }
        }
    }

    macro_rules! emit {
        ($($sev: ident => $method: ident),*) => {
            match sev {
                $($sev => cx.$method(sp, accumulator.as_slice()),)*
            }
        }
    }
    emit! {
        Note => span_note,
        Warning => span_warn,
        Error => span_err,
        Fatal => span_fatal
    }

    DummyResult::any(sp)
}
