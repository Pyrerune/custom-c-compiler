use std::process::exit;
use std::str::Chars;

pub trait ToDigit {
    fn to_digit(&self) -> i32;
}
pub trait Operators {
    fn contains_operator(&self) -> bool;
    fn find_each_operator(&self) -> Vec<usize>;
}
impl Operators for Vec<String> {
    fn contains_operator(&self) -> bool {

        let mut ret = false;
        for i in self {
            match i.as_str() {
                "+" | "-" => {
                    ret = true;

                },
                _ => {},
            }
        }
        ret
    }
    fn find_each_operator(&self) -> Vec<usize> {
        let mut n = 0;
        let mut v = vec![];
        for i in 0..self.len() {
            if self[i] == "+".to_string() || self[i] == "-".to_string() {

                v.push(i.clone());
            }
        }
        v
    }
}
impl ToDigit for String {
    fn to_digit(&self) -> i32 {
        let chars = self.chars().collect::<Vec<char>>();
        let mut num = 0;
        for i in 0..chars.len() {
            let pow = (chars.len()-1) - i;
            if chars[i].is_digit(10) {
                let j = chars[i].to_digit(10).unwrap();
                let multiplier = 10i32.pow(pow as u32);

                num += j as i32*multiplier;
            }
        }
        num
    }

}
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
#[derive(Debug, Clone)]
pub enum Expression {
    Constant(i32),
    UnOp(String, i32),
    BinOp(String, Box<Expression>, Box<Expression>),
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

        //rewrite 2
        /*let ftype = iter.next();
        let _ = iter.next();
        let fname = iter.next();
        let _ = iter.next();
        let args = iter.next();
        let _ = iter.next();
        let statement = iter.next();

        let unop1 = iter.next();
        let exp1 = iter.next();
        let binop = iter.next();
        let unop2 = iter.next();
        let exp2 = iter.next();*/
        let mut root = Function {
            name: "".to_string(),
            ftype: "".to_string(),
            value: Statement { name: "".to_string(), value: Expression::Constant(0) }
        };
        let mut statement = Statement {
            name: "".to_string(),
            value: Expression::Constant(0),
        };
        let mut ints = vec![];
        for i in &self.token_list {
            match i.as_str() {
                "NAME" | "WITH ARGS" | "BODY" => {}
                _ => {
                    if root.ftype.is_empty() {
                        root.ftype = i.clone();
                    } else if root.name.is_empty() {
                        root.name = i.clone();
                    } else if statement.name.is_empty() {
                        statement.name = i.clone();
                    } else {
                        ints.push(i.clone());
                    }
                }
            }
        }
        /*let mut v = vec![];
        if ints.contains_operator() {
            let n = ints.find_each_operator();
            let binop =  &mut Expression::BinOp(String::new(), Box::from(Expression::Constant(0)),  Box::from(Expression::Constant(0)));
            while ints.len() > 0 {
                if let Expression::BinOp(x, _, z) = binop {
                    println!("{:?}", ints.clone());
                    *z = Box::from(Expression::Constant(ints[ints.len()-1].to_digit()));
                    ints.pop();
                    println!("{:?}", ints.clone());
                    if ints.len() > 0 {
                        *x = ints[ints.len() - 1].clone();
                        ints.pop();
                    }
                }
                v.push(binop.clone().to_owned());
            }
        }

        //let mut w = vec![];
        for i in 0..(v.len()-2) {
            if let Expression::BinOp(_, x, _) = v[v.len()-1] {
                if let Expression::BinOp(_, _, y) = v[v.len()-2] {\
                }

            }
        }*/
        //rewrite 2
        /*if fname.is_some() && ftype.is_some() && args.is_some() && statement.is_some() && unop1.is_some() && exp1.is_some() && binop.is_some() && unop2.is_some()&& exp2.is_some() {
            root.name = fname.unwrap().clone();
            root.ftype = ftype.unwrap().clone();
            root.value.name = statement.unwrap().clone();

                //rewrite 2
               /* if !binop.unwrap().is_empty() {
                    if !unop1.unwrap().is_empty() {
                        if !unop2.unwrap().is_empty() {
                            root.value.value = Expression::BinOp(binop.unwrap().clone(), Box::from(Expression::UnOp(unop1.unwrap().clone(), exp1.unwrap().clone().to_digit())), Box::from(Expression::UnOp(unop2.unwrap().clone(), exp2.unwrap().clone().to_digit())));
                        } else {
                            root.value.value = Expression::BinOp(binop.unwrap().clone(), Box::from(Expression::UnOp(unop1.unwrap().clone(), exp1.unwrap().clone().to_digit())), Box::from(Expression::Constant(exp2.unwrap().clone().to_digit())));
                        }
                    } else {
                        if !unop2.unwrap().is_empty() {
                            root.value.value = Expression::BinOp(binop.unwrap().clone(), Box::from(Expression::Constant(exp1.unwrap().clone().to_digit())), Box::from(Expression::UnOp(unop2.unwrap().clone(), exp2.unwrap().clone().to_digit())));
                        } else {
                            root.value.value = Expression::BinOp(binop.unwrap().clone(), Box::from(Expression::Constant(exp1.unwrap().clone().to_digit())), Box::from(Expression::Constant(exp2.unwrap().clone().to_digit())));
                        }
                    }

                } else {
                    root.value.value = Expression::Constant(exp1.unwrap().clone().to_digit());
                }*/

            //rewrite
            /*if unop_value.is_none() {
                root = Function {
                    name: fname.unwrap().to_owned(),
                    ftype: ftype.unwrap().to_owned(),
                    value: Statement {
                        name: statement.unwrap().to_owned(),
                        value: Expression::Constant(to_digit(statement_value.unwrap().chars().collect::<Vec<char>>())),
                    }
                };
            } else {
                root = Function {
                    name: fname.unwrap().to_owned(),
                    ftype: ftype.unwrap().to_owned(),
                    value: Statement {
                        name: statement.unwrap().to_owned(),
                        value: Expression::Constant(to_digit(unop_value.unwrap().chars().collect::<Vec<char>>())),
                    }
                };
            }

            if unop.is_some() && unop_value.is_some() {
                println!("1. {:?} {:?}", unop, unop_value);
                root = Function {
                    name: fname.unwrap().to_owned(),
                    ftype: ftype.unwrap().to_owned(),
                    value: Statement {
                        name: statement.unwrap().to_owned(),
                        value: Expression::UnOp(unop.unwrap().to_owned(), to_digit(unop_value.unwrap().chars().collect::<Vec<char>>()))
                    }
                };
            }
            */

            //TODO This is pretty much completely optional
            if fname.unwrap().is_empty() || ftype.unwrap().is_empty() || statement.unwrap().is_empty() || exp1.unwrap().is_empty() {
                result = Err("ERROR: Lexer failed, please check that your code was tokenized properly or file a bug with the developer");
            } else {
                result = Ok(root);
            }
        } else {
            result = Err("ERROR: Lexer failed, please check that your code was tokenized properly or file a bug with the developer");
        }*/
        //result
        Err("Testing Phase")
    }
}
impl Lexer {
    pub fn new(s: String) -> Lexer {
        Lexer {
            token_list: s
        }
    }
    pub fn lex(&self) -> Vec<String> {

        let mut state = 0;
        let mut lexed_tokens: Vec<String> = vec![];
        let split: Vec<char> = self.token_list.chars().collect();
        let mut token = String::new();
        for i in 0..split.len() {
          //  println!("{}", token);
            if (token.contains(" ") || token.contains("\n")) && state != 1 {
                token.clear();
            } else if token == "int".to_string() {
                //println!("INT");
                lexed_tokens.push(String::from("INT"));
                state = 1;
                token.clear();
            } else if token.contains("(") {
                //println!("token: {}", token);
                //TODO check this
                if state == 1 {
                    token.remove(token.find("(").unwrap());
                    while token.contains(" ") {
                        token.remove(token.find(" ").unwrap());
                    }
                    while token.contains("\n") {
                        token.remove(token.find("\n").unwrap());
                    }
                    //println!("token: {}", token);
                    lexed_tokens.push(String::from("NAME"));
                    lexed_tokens.push(token.clone());
                    state = 2;
                }
                token.clear();
            } else if token.contains(")") {
                //TODO check this
                if state == 2 {
                    token.remove(token.find(")").unwrap());
                    lexed_tokens.push(String::from("WITH ARGS"));
                    lexed_tokens.push(token.clone());
                    state = 0;
                }
                token.clear();
            } else if token == "{".to_string() {
                lexed_tokens.push(String::from("BODY"));
                token.clear();
            } else if token == "}".to_string() {
                lexed_tokens.push(String::from("END"));
                token.clear();
            } else if token == "return".to_string() {
                lexed_tokens.push(String::from("RETURN"));

                let mut j = i.clone();

                while split[j] != ';' {
                    if split[j] != ' ' {
                        lexed_tokens.push(split[j].to_string());
                    }
                    if split.len() > j + 1 {
                        j += 1;
                    }
                }
            }
            token.push(split[i].clone());
        }
        println!("{:?}", lexed_tokens.clone());
        lexed_tokens
    }
}


pub fn generate(f: Function) -> String {
    //TODO add binary operator to asm
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
    } else if let Expression::BinOp(x, y, z) = sexp {
        ret_val = parse_int(x, y, z);
    }

    let mut asm = String::new();
    asm.push_str("\t.globl ");
    asm.push_str(&fname.as_str());
    asm.push_str("\n");
    asm.push_str("\t.type ");
    asm.push_str(&fname.as_str());
    asm.push_str(", @function");
    asm.push_str("\n");
    asm.push_str(&fname.as_str());
    asm.push_str(":\n");
    asm.push_str("\tmovl $");
    asm.push_str(&ret_val.to_string().as_str());
    asm.push_str(", %eax\n");
    if unop_op.is_some() {
        let mut split = unop_op.unwrap().chars().collect::<Vec<char>>();
        split.reverse();
        for i in split {

            match i {
                '-' => {
                    asm.push_str("\tneg %eax\n");
                }
                '~' => {
                    asm.push_str("\tnot %eax\n");
                }
                '!' => {
                    asm.push_str("\tcmpl $0, %eax\n");
                    asm.push_str("\tmovl $0, %eax\n");
                    asm.push_str("\tsete %al\n");
                }
                _ => {
                }
            }
        }
    }

    asm.push_str("\tret\n");
    asm
}

fn is_unary_operator(c: char) -> bool {
    //let mut operator = false;
    return match c {
        '!' => true,
        '-' => true,
        '~' => true,
        _ => false,
    }
}

fn is_binary_operator(c: char) -> bool {
    return match c {
        '+' => true,
        '-' => true,
        '/' => true,
        '*' => true,
        _ => false,
    }
}
fn parse_int(operator: String, digit_1: Box<Expression>, digit_2: Box<Expression>) -> i32 {
    let exp1 = digit_1.as_ref();
    let exp2 = digit_2.as_ref();
    let mut d_1 = 0;
    let mut d_2 = 0;
    let mut ret = 0;
    if let Expression::Constant(x) = exp1 {
     d_1 = x.clone();
    }
    if let Expression::Constant(x) = exp2 {
        d_2 = x.clone();
    }
    match operator.as_str() {
        "+" => {
            ret = d_1+d_2;
        }
        _ => {}
    }

    ret
}