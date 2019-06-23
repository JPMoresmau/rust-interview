

#[derive(Debug)]
pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide
}

#[derive(Debug)]
pub enum Node {
    Value(f64),
    SubNode(Box<Node>),
    Binary(Operator, Box<Node>,Box<Node>),
}

pub fn parse(txt :&str) -> Option<Node> {
    let chars = txt.chars().filter(|c| *c != ' ').collect();
    parse_expression(&chars, 0).map(|(_,n)| n)
}

fn parse_expression(chars: &Vec<char>, pos: usize) -> Option<(usize,Node)> {
    match parse_start(chars, pos) {
        Some((new_pos, first)) => {
            match parse_operator(chars, new_pos) {
                Some((new_pos2,op)) => {
                    if let Some((new_pos3, second)) = parse_expression(chars, new_pos2) {
                        Some((new_pos3, combine(op, first, second)))
                    } else {
                        None
                    }
                    
                },
                None => Some((new_pos,first)), 
            }
        },
        None => None,
    }
}

fn combine(op: Operator, first: Node, second: Node) -> Node {
    match second {
        Node::Binary(op2,v21,v22) => if precedence(&op)>=precedence(&op2) {
            Node::Binary(op2,Box::new(combine(op,first,*v21)),v22)
        } else {
            Node::Binary(op,Box::new(first),Box::new(Node::Binary(op2,v21,v22)))
        },
        _ => Node::Binary(op,Box::new(first),Box::new(second)),
    }
}

fn precedence(op: &Operator) -> usize{
    match op{
        Operator::Multiply | Operator::Divide => 2,
        _ => 1
    }
}

fn parse_start(chars: &Vec<char>, pos: usize) -> Option<(usize,Node)> {
    match start_parenthesis(chars, pos){
        Some (new_pos) => {
            let r = parse_expression(chars, new_pos);
            end_parenthesis(chars, r)
        },
        None => parse_value(chars, pos),
    }
}

fn start_parenthesis(chars: &Vec<char>, pos: usize) -> Option<usize>{
    if pos<chars.len() && chars[pos] == '(' {
        Some(pos+1)
    } else {
        None
    }
}

fn end_parenthesis(chars: &Vec<char>, wrapped :Option<(usize,Node)>) -> Option<(usize,Node)>{
    match wrapped {
        Some((pos, node)) => if pos<chars.len() && chars[pos] == ')' {
                Some((pos+1,Node::SubNode(Box::new(node))))
            } else {
                None
            },
        None => None,
    }
}

fn parse_value(chars: &Vec<char>, pos: usize) -> Option<(usize,Node)>{
    let mut new_pos = pos;
    if new_pos<chars.len() && chars[new_pos] == '-' {
        new_pos = new_pos+1;
    }
    while new_pos<chars.len() && (chars[new_pos]=='.' || (chars[new_pos] >= '0' && chars[new_pos] <= '9')) {
        new_pos = new_pos+1;
    }
    if new_pos>pos {
        if let Ok(v) = dbg!(chars[pos..new_pos].iter().collect::<String>()).parse() {
            Some((new_pos,Node::Value(v)))
        } else {
            None
        }
    } else {
        None
    }

}


fn parse_operator(chars: &Vec<char>, pos: usize) -> Option<(usize,Operator)> {
    if pos>=chars.len() {
        None
    } else if chars[pos] == '+' {
        Some((pos+1,Operator::Add))
    } else if chars[pos] == '-' {
        Some((pos+1,Operator::Substract))
    } else if chars[pos] == '*' {
        Some((pos+1,Operator::Multiply))
    } else if chars[pos] == '/' {
        Some((pos+1,Operator::Divide))
    } else {
        None
    }
}

pub fn eval(txt :&str) -> f64 {
    let ot = dbg!(parse(txt));
    match ot {
        Some(t) => eval_term(&t),
        None => panic!("Cannot parse {}",txt),
    }
    
}

fn eval_term(t: &Node) -> f64 {
    match t {
        Node::Value(v) => *v,
        Node::SubNode(t) => eval_term(t),
        Node::Binary(Operator::Add,t1,t2) => eval_term(t1) + eval_term(t2),
        Node::Binary(Operator::Substract,t1,t2) => eval_term(t1) - eval_term(t2),
        Node::Binary(Operator::Multiply,t1,t2) => eval_term(t1) * eval_term(t2),
        Node::Binary(Operator::Divide,t1,t2) => eval_term(t1) / eval_term(t2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval(){
        assert_eq!(2.0,eval("2"));
        assert_eq!(4.0,eval("2+2"));
        assert_eq!(11.0/4.0, eval("2+3/4"));
        assert_eq!(2.0, eval("2*3-4"));
        assert_eq!(89.0/6.0, eval("2*(3+4)+5/6"));
        assert_eq!(14.0, eval("2 * (3 -1) + 2 * 5"));
        assert_eq!(7000.0, eval("2 * (3 + (4 * 5 + (6 * 7) * 8) - 9) * 10"));
        assert_eq!(-9.0/4.0, eval("2*-3--4+-.25"));
        assert_eq!(1.5, eval("1 - 5 * 2 / 20 + 1"));
        assert_eq!(3.5, eval("2 * (3 + ((5) / (7 - 11)))"));
        
    }
}