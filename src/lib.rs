#[macro_use]
mod serr;
#[macro_use]
mod utils;

mod env;
mod lexer;
mod parser;
mod expander;
mod port;
mod procedure;
mod evaluator;
mod primitives;
mod pretty_print;
mod repl;

use env::{Env, EnvRef};
use lexer::tokenize;
use parser::parse;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Scheme {
    i: u32,
    env: EnvRef,
}

#[wasm_bindgen]
impl Scheme {
    pub fn new() -> Scheme {
        
        let env = Env::with_values(EnvRef::null(), primitives::env()).into_ref();

        match primitives::load_prelude(&env) {
            Err(e) => log!("{}", e),
            _ => (),
        }

        Scheme {
            i: 0,
            env: env,
        }
    }

    pub fn evaluate(&mut self, input: Option<String>) {
        match input {
            Some(line) => {
                let tokens = lexer::tokenize(&mut line.chars().peekable());
                let sexprs = parser::parse(tokens);

                match sexprs {
                    Ok(sexprs) => {
                        for sexpr in sexprs {
                            let evaluated = sexpr.eval(&self.env);

                            match evaluated {
                                Ok(evaluated) => {
                                    if !evaluated.is_unspecified() {
                                        log!("${} = {}", self.i, evaluated);
                                        self.env.define(format!("${}", self.i), evaluated);
                                        self.i += 1;
                                    }
                                },
                                Err(e) => log!("{}", e)
                            }
                        }
                    },
                    Err(e) => log!("{}", e)
                }
            }
            None => log!("no input!")
        }        
    }
}
