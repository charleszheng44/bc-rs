macro_rules! some_box {
    ($a: expr) => {
        Some(Box::new($a))
    };
}

macro_rules! oper_ast {
    ($o: expr, $l: expr, $r: expr) => {
        AST {
            operator: Some($o),
            left_operand: some_box!($l),
            right_operand: some_box!($r),
            ..Default::default()
        }
    };
}

macro_rules! num_ast {
    ($a: expr) => {
        AST {
            num: Some($a),
            ..Default::default()
        }
    };
}

macro_rules! some_num_ast {
    ($a: expr) => {
        Some(num_ast!($a))
    };
}

macro_rules! some_box_num_ast {
    ($a: expr) => {
        some_box!(num_ast!($a))
    };
}

macro_rules! some_oper_ast_left {
    ($o: expr, $l: expr) => {
        Some(AST {
            operator: Some($o),
            left_operand: some_box!($l),
            ..Default::default()
        })
    };
}

pub(crate) use {
    num_ast, oper_ast, some_box, some_box_num_ast, some_num_ast,
    some_oper_ast_left,
};
