#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Angka(f64),
    Teks(String),
    Bolean(bool),
    Kosong,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub default: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Tambah,     // +
    Kurang,     // -
    Kali,       // *
    Bagi,       // /
    Modulo,     // %
    Pangkat,    // **
    SamaDengan, // ==, sama_dengan
    TidakSama,  // !=
    KurangDari, // <, kurang_dari
    KurangSama, // <=
    LebihDari,  // >, lebih_dari
    LebihSama,  // >=
    Dan,        // dan, &&
    Atau,       // atau, ||
    Dalam,      // dalam (in)
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus, // -
    Bukan, // bukan, !
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOp {
    Assign,       // =
    TambahAssign, // +=
    KurangAssign, // -=
    KaliAssign,   // *=
    BagiAssign,   // /=
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForIter {
    Range { start: Expr, end: Expr },
    Collection(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal {
        value: LiteralValue,
        line: usize,
    },
    Identifier {
        name: String,
        line: usize,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        line: usize,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        line: usize,
    },
    Assign {
        target: Box<Expr>,
        op: AssignOp,
        value: Box<Expr>,
        line: usize,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        line: usize,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
        end: Option<Box<Expr>>,
        step: Option<Box<Expr>>,
        line: usize,
    },
    Property {
        target: Box<Expr>,
        name: String,
        line: usize,
    },
    ArrayLiteral {
        elements: Vec<Expr>,
        line: usize,
    },
    DictLiteral {
        entries: Vec<(Expr, Expr)>,
        line: usize,
    },
    FuncExpr {
        params: Vec<Param>,
        body: Block,
        line: usize,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDecl {
        name: String,
        init: Option<Expr>,
        is_const: bool,
        line: usize,
    },
    FuncDecl {
        name: String,
        params: Vec<Param>,
        body: Block,
        line: usize,
    },
    IfStatement {
        condition: Expr,
        then_branch: Block,
        elif_branches: Vec<(Expr, Block)>,
        else_branch: Option<Block>,
        line: usize,
    },
    MatchStatement {
        expr: Expr,
        cases: Vec<(Expr, Block)>,
        wildcard: Option<Block>,
        default: Option<Block>,
        line: usize,
    },
    WhileStatement {
        condition: Expr,
        body: Block,
        line: usize,
    },
    ForStatement {
        var_name: String,
        iter: ForIter,
        body: Block,
        line: usize,
    },
    ReturnStatement {
        value: Option<Expr>,
        line: usize,
    },
    BreakStatement {
        line: usize,
    },
    ContinueStatement {
        line: usize,
    },
    HapusStatement {
        target: Expr,
        line: usize,
    },
    TryCatchStatement {
        try_block: Block,
        catch_var: String,
        catch_block: Block,
        finally_block: Option<Block>,
        line: usize,
    },
    ThrowStatement {
        expr: Expr,
        line: usize,
    },
    AssertStatement {
        condition: Expr,
        message: Option<Expr>,
        line: usize,
    },
    ImportStatement {
        module: String,
        alias: Option<String>,
        items: Option<Vec<String>>,
        line: usize,
    },
    ExprStatement {
        expr: Expr,
        line: usize,
    },
    Block(Block),
}
