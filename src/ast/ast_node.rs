use std::collections::LinkedList;

#[derive(Debug)]
pub struct Prog {
    pub p_main: Proc,
    pub ps: LinkedList<Proc>,
}

#[derive(Debug)]
pub enum Vdec {
    Scalar(Var),
    Array { x: Var, c: Con },
}

#[derive(Debug)]
pub enum Type {
    Int,
    Stack,
}

#[derive(Debug)]
pub struct Arg {
    pub t: Type,
    pub x: Var,
}

#[derive(Debug)]
pub enum MainStuff {
    Int(Vdec),
    Stack(Var),
}

#[derive(Debug)]
pub enum Proc {
    Main {
        main_stuff: LinkedList<MainStuff>,
        s: Stm,
    },
    Other {
        q: PId,
        args: LinkedList<Arg>,
        s: Stm,
    },
}

#[derive(Debug)]
pub enum Stm {
    AssignScalar {
        x: Var,
        mod_op: ModOp,
        e: Exp,
    },
    AssignArray {
        x: Var,
        e_index: Exp,
        mod_op: ModOp,
        e: Exp,
    },
    Conditional {
        e_if: Exp,
        s_then: Box<Stm>,
        s_else: Box<Stm>,
        e_fi: Exp,
    },
    Loop {
        e_from: Exp,
        s_do: Box<Stm>,
        s_loop: Box<Stm>,
        e_until: Exp,
    },
    Push(Var, Var),
    Pop(Var, Var),
    Local {
        t_local: Type,
        x_local: Var,
        e_local: Exp,
        s: Box<Stm>,
        t_delocal: Type,
        x_delocal: Var,
        e_delocal: Exp,
    },
    Call {
        q: PId,
        xs: LinkedList<Var>,
    },
    Uncall {
        q: PId,
        xs: LinkedList<Var>,
    },
    Skip,
    Sequence(Box<Stm>, Box<Stm>),
}

#[derive(Debug)]
pub enum Exp {
    Constant(Con),
    Variable(Var),
    Indexed { x: Var, e: Box<Exp> },
    BinOp(Box<Exp>, Op, Box<Exp>),
    Empty(Var),
    Top(Var),
    Nil,
}

#[derive(Debug)]
pub struct Con(pub i32);

#[derive(Debug)]
pub enum ModOp {
    Add,
    Sub,
    Xor,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Xor,
    Mul,
    Div,
    Mod,
    And,
    Or,
    And2,
    Or2,
    Less,
    Greater,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug)]
pub struct Var(pub String);

#[derive(Debug)]
pub struct PId(pub String);
