use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct NodeProg {
    pub functions: Vec<NodeFunc>,
}

impl Debug for NodeProg {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Prog")?;
        for (idx, func) in self.functions.iter().enumerate() {
            write!(f, " func_{}={:?}", idx, func)?;
        }
        write!(f, ">")
    }
}

pub struct NodeFunc {
    pub ident: NodeIdent,
    pub r_type: NodeType,
    pub block: NodeBlock,
}

impl Debug for NodeFunc {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "<Func {:?} r_type={:?} block={:?}>",
            self.ident, self.r_type, self.block
        )
    }
}

pub struct NodeBlock {
    pub stmts: Vec<NodeStmt>,
}

impl Debug for NodeBlock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Block")?;
        for (idx, stmt) in self.stmts.iter().enumerate() {
            write!(f, " stmt_{}={:?}", idx, stmt)?;
        }
        write!(f, ">")
    }
}

pub enum NodeStmt {
    Return(NodeExpr),
    Assign(NodeIdent, NodeType, NodeExpr),
}

impl Debug for NodeStmt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NodeStmt::Return(expr) => write!(f, "<Return expr={:?}>", expr),
            NodeStmt::Assign(ident, a_type, expr) => {
                write!(
                    f,
                    "<Assign {:?} a_type={:?} expr={:?}>",
                    ident, a_type, expr
                )
            }
        }
    }
}

pub enum NodeExpr {
    Literal(NodeLiteral),
    Ident(NodeIdent),
}

impl Debug for NodeExpr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NodeExpr::Literal(literal) => write!(f, "<Literal {:?}>", literal),
            NodeExpr::Ident(ident) => write!(f, "<Ident {:?}>", ident),
        }
    }
}

pub enum NodeLiteral {
    IntLit(i64),
}

impl Debug for NodeLiteral {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NodeLiteral::IntLit(x) => write!(f, "type=int value={}", x),
        }
    }
}

pub struct NodeIdent {
    pub name: String,
}

impl Debug for NodeIdent {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "name={}", self.name)
    }
}

pub struct NodeType {
    pub meta: TypeMeta,
}

impl Debug for NodeType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Type meta={:?}>", self.meta)
    }
}

pub enum TypeMeta {
    Primitive(PrimitiveType),
}

impl Debug for TypeMeta {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TypeMeta::Primitive(primitive_type) => write!(f, "{:?}", primitive_type),
        }
    }
}

pub enum PrimitiveType {
    Int,
}

impl Debug for PrimitiveType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            PrimitiveType::Int => write!(f, "int"),
        }
    }
}
