// Copyright 2023 Sean Kelleher. All rights reserved.
// Use of this source code is governed by an MIT
// licence that can be found in the LICENCE file.

#[derive(Clone, Debug)]
pub enum Prog {
    Body{stmts: Block},
}

pub type Block = Vec<Stmt>;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block{block: Block},

    Expr{expr: Expr},

    Declare{lhs: Expr, rhs: Expr},
    Assign{lhs: Expr, rhs: Expr},

    Func{name: (String, Location), args: Vec<Expr>, stmts: Block},
}

pub type Location = (usize, usize);

pub type Expr = (RawExpr, Location);

#[derive(Clone, Debug)]
pub enum RawExpr {
    Null,

    Bool{b: bool},
    Int{n: i64},
    Str{s: String},

    Var{name: String},

    BinaryOp{
        op: BinaryOp,
        op_loc: Location,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    List{items: Vec<Expr>},
    Object{props: Vec<PropItem>},

    Call{func: Box<Expr>, args: Vec<Expr>},
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
    Sum,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Clone, Debug)]
pub enum PropItem {
    Pair{name: Expr, value: Expr},
}
