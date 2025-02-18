use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct NodeProgram {
    functions: Vec<NodeFunc>,
}

impl Debug for NodeProgram {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut res = "<Program ".to_string();
        for (idx, function) in self.functions.iter().enumerate() {
            res.push_str(format!("function_{}={:?} ", idx, function).as_str());
        }
        write!(f, "{}>", res[..res.len() - 1].to_string())
    }
}

impl NodeProgram {
    pub fn new(functions: Vec<NodeFunc>) -> NodeProgram {
        NodeProgram { functions }
    }
}

pub struct NodeFunc {
    identifier: NodeIdent,
    // parameters: Vec<NodeParameter>,
    return_type: NodeType,
    body: NodeBlock,
}

impl Debug for NodeFunc {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "<Function identifier={:?} rtype={:?} body={:?}>",
            self.identifier, self.return_type, self.body
        )
    }
}

impl NodeFunc {
    pub fn new(identifier: NodeIdent, return_type: NodeType, body: NodeBlock) -> NodeFunc {
        NodeFunc {
            identifier,
            return_type,
            body,
        }
    }
}

pub struct NodeType {
    name: String,
}

impl Debug for NodeType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Type name={:?}>", self.name)
    }
}

impl NodeType {
    pub fn new(name: String) -> NodeType {
        NodeType { name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct NodeBlock {
    statements: Vec<NodeStatement>,
}

impl Debug for NodeBlock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut res = "<Block ".to_string();
        for (idx, statement) in self.statements.iter().enumerate() {
            res.push_str(format!("statement_{}={:?} ", idx, statement).as_str());
        }
        write!(f, "{}>", res[..res.len() - 1].to_string())
    }
}

impl NodeBlock {
    pub fn new(statements: Vec<NodeStatement>) -> NodeBlock {
        NodeBlock { statements }
    }
}

pub enum NodeStatement {
    Return(NodeReturn),
}

impl Debug for NodeStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NodeStatement::Return(r) => write!(f, "{:?}", r),
        }
    }
}

pub struct NodeReturn {
    expression: NodeExpr,
}

impl Debug for NodeReturn {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Return expression={:?}>", self.expression)
    }
}

impl NodeReturn {
    pub fn new(expression: NodeExpr) -> NodeReturn {
        NodeReturn { expression }
    }
}

pub enum NodeExpr {
    Id(NodeIdent),
    Literal(NodeLiteral),
}

impl Debug for NodeExpr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NodeExpr::Id(i) => write!(f, "{:?}", i),
            NodeExpr::Literal(l) => write!(f, "{:?}", l),
        }
    }
}

pub struct NodeIdent {
    name: String,
    _metatype: String,
}

impl Debug for NodeIdent {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Identifier name={:?}>", self.name)
    }
}

impl NodeIdent {
    pub fn new(name: String, possible_type: Option<&String>) -> NodeIdent {
        NodeIdent {
            name,
            _metatype: match possible_type {
                Some(t) => t.clone(),
                None => "void".to_string(),
            },
        }
    }

    pub fn set_metatype(&mut self, metatype: String) {
        self._metatype = metatype;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct NodeLiteral {
    value: String,
    _metatype: String,
}

impl Debug for NodeLiteral {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Literal value={:?}>", self.value)
    }
}

impl NodeLiteral {
    pub fn new(value: String) -> NodeLiteral {
        NodeLiteral {
            value,
            _metatype: "int".to_string(),
        }
    }
}
