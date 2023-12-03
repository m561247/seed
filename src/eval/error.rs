// Copyright 2023 Sean Kelleher. All rights reserved.
// Use of this source code is governed by an MIT
// licence that can be found in the LICENCE file.

use snafu::Snafu;

use eval::Value;

// TODO Ideally `Error` would be defined in `src/eval/mod.rs`, since these are
// errors that occur during evaluation. However, we define it here because
// `value::Value::BuiltInFunc` refers to it. We could make the error type for
// `value::Value::BuiltInFunc` generic, but this generic type would spread
// throughout the codebase for little benefit, so we take the current approach
// for now.
#[derive(Clone, Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    // TODO Consider adding a rendered version of the source expression to
    // highlight what the interpreter attempted to evaluate.
    #[snafu(display("value is not a function"))]
    CannotCallNonFunc{v: Value},
    #[snafu(display("'{}' is not defined", name))]
    Undefined{name: String},
    #[snafu(display("cannot bind to {}", descr))]
    InvalidBindTarget{descr: String},
    #[snafu(display("'{}' is bound multiple times in this binding", name))]
    AlreadyInBinding{name: String},
    #[snafu(display("'{}' is already defined in the current scope", name))]
    AlreadyInScope{name: String},

    #[snafu(display("{}", msg))]
    BuiltinFuncErr{msg: String},

    // NOTE This is a somewhat hacky way of adding location information to
    // errors in a generic way. Ideally this information could be better
    // decoupled from the core error type, but we take this approach for now
    // for simplicity.
    #[snafu(display("{}:{}: {}", line, col, source))]
    AtLoc{
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
        line: usize,
        col: usize,
    },

    EvalProgFailed{
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },
    EvalStmtsInNewScopeFailed{
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },
    EvalStmtsWithScopeStackFailed{
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },
    EvalStmtsFailed{
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },
    EvalStmtFailed{
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },
}