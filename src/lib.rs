use std::process::exit;
use std::str::Chars;
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub ftype: String,
    pub value: Statement,
}

#[derive(Debug)]
pub struct Statement {
    pub name: String,
    pub value: Expression,
}
#[derive(Debug)]
pub enum Expression {
    Constant(i32),
    UnOp(String, i32),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Lexer {
    pub token_list: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Parser {
    pub token_list: Vec<String>,
}
impl Parser {
    pub fn new(s: Vec<String>) -> Parser {
        Parser {
            token_list: s
        }
    }
    pub fn parse(&self) -> Result<Function, &str>{
        let mut result: Result<Function, &str>;
        let mut iter = self.token_list.iter();
        let ftype = iter.next();
        let _ = iter.next();
        let fname = iter.next();
        let _ = iter.next();
        let args = iter.next();
        let _ = iter.next();
        let statement = iter.next();
        let statement_value = iter.next();
        let unop = iter.next();
        let unop_value = iter.next();
        if fname.is_some() && ftype.is_some() && args.is_some() && statement.is_some() && statement_value.is_some() {
            let mut root = Function {
                name: fname.unwrap().to_owned(),
                ftype: ftype.unwrap().to_owned(),
                value: Statement {
                    name: statement.unwrap().to_owned(),
                    value: Expression::Constant(0),
                }
            };
            if unop.is_some() && unop_value.is_some() {
                root = Function {
                    name: fname.unwrap().to_owned(),
                    ftype: ftype.unwrap().to_owned(),
                    value: Statement {
                        name: statement.unwrap().to_owned(),
                        value: Expression::UnOp(unop.unwrap().to_owned(), to_digit(unop_value.unwrap().chars().collect::<Vec<char>>()))
                    }
                };
            }

            result = Ok(root);
        } else {
            result = Err("ERROR: Lexer failed, please check that your code was tokenized properly or file a bug with the developer");
        }
        result
    }
}
impl Lexer {
    pub fn new(s: String) -> Lexer {
        Lexer {
            token_list: s
        }
    }
    pub fn lex(&self) -> Vec<String> {
        let mut lexed_tokens: Vec<String> = vec![];
        let split: Vec<char> = self.token_list.chars().collect();
        let mut token = String::new();
        for i in 0..split.len() {
            //println!("{}", token);
            if token.contains(" ") || token.contains("\n"){
                token.clear();
            } else if token == "int".to_string() {
                lexed_tokens.push(String::from("INT"));
                token.clear();
            } else if token.contains("(") {
                token.remove(token.find("(").unwrap());
                lexed_tokens.push(String::from("NAME"));
                lexed_tokens.push(token.clone());
                token.clear();
            } else if token.contains(")") {
                token.remove(token.find(")").unwrap());
                lexed_tokens.push(String::from("WITH ARGS"));
                lexed_tokens.push(token.clone());
                token.clear();
            } else if token == "{".to_string() {
                lexed_tokens.push(String::from("BODY"));
                token.clear();
            } else if token == "}".to_string() {
                lexed_tokens.push(String::from("END"));
                token.clear();
            } else if token == "return".to_string() {
                lexed_tokens.push(String::from("RETURN"));
                if split.len() > i+1 {

                    if split[i+1].is_ascii_digit() {
                        lexed_tokens.push(split[i + 1].to_string().clone());
                    } else {
                        if is_operator(split[i+1]) {
                            lexed_tokens.push(String::from("OPERATOR"));
                            lexed_tokens.push(split[i+1].to_string());
                            if split[i+2].is_ascii_digit() {
                                lexed_tokens.push(split[i+2].to_string());
                            } else {
                                eprintln!("ERROR: Operator present without integer");
                                exit(1);
                            }
                        }
                    }
                } else {
                    eprintln!("ERROR: keyword return requires a value");
                    exit(1);
                }
                token.clear();
            }
            token.push(split[i].clone());

        }
        //println!("{:?}", lexed_tokens.clone());
        lexed_tokens
    }
}

pub fn to_digit(s: Vec<char>) -> i32 {
    let mut num = 0;
    for i in 0..s.len() {
        num += s[i].to_digit(10).unwrap();
    }
    num as i32
}

pub fn generate(f: Function) -> String {
    let fname = f.name;
    let ftype = f.ftype;
    let fbody = f.value;
    let sname = fbody.name;
    let sexp = fbody.value;
    let mut unop_op: Option<String> = None;
    let mut ret_val: i32 = 0;
    if let Expression::UnOp(x, y) = sexp {
        unop_op = Some(x);
        ret_val = y;
    } else if let Expression::Constant(x) = sexp {
        ret_val = x;
    }

    let mut asm = String::new();
    asm.push_str(".globl ");
    asm.push_str(&fname.as_str());
    asm.push_str("\n");
    asm.push_str(".type ");
    asm.push_str(&fname.as_str());
    asm.push_str(", @function");
    asm.push_str("\n");
    asm.push_str(&fname.as_str());
    asm.push_str(":\n");
    asm.push_str("movl $");
    asm.push_str(&ret_val.to_string().as_str());
    asm.push_str(", %eax\n");
    if unop_op.is_some() {
        match unop_op.unwrap().as_str() {
            "-" => {
                asm.push_str("neg %eax\n");
            }
            "~" => {
                asm.push_str("not %eax\n");
            }
            "!" => {
                asm.push_str("cmpl $0, %eax\n");
                asm.push_str("movl $0, %eax\n");
                asm.push_str("sete %al\n");
            }
            _ => {}
        }
    }

    asm.push_str("ret\n");
    asm
}

fn is_operator(c: char) -> bool {
    //let mut operator = false;
    return match c {
        '!' => true,
        '-' => true,
        '~' => true,
        _ => false,
    }
}