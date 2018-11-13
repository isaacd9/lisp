#[derive(Debug)]
#[derive(Clone)]
enum Operator {
    Plus
}

#[derive(Debug)]
#[derive(Clone)]
enum Token {
    LeftParen,
    RightParen,
    Int(i32),
    Operator(Operator),
    Unknown,
}

fn tokenize_operator(st: &str) -> Token {
    return match st {
        "+" => Token::Operator(Operator::Plus),
        _ => Token::Unknown
    }
}

fn tokenize(st: &str) -> Vec<Token> {
    let replaced = st.replace("(", " ( ").replace(")", " ) ");
    return replaced.split_whitespace().map(|ch| {
        match ch.chars().next().unwrap() {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '+' | '-' | '*' | '/' => tokenize_operator(ch),
            '0' ... '9' => match ch.parse::<i32>() {
                Ok(i) => Token::Int(i),
                Err(_) => Token::Unknown,
            }
            _ => Token::Unknown,
        }
    }).collect();
}

#[derive(Debug)]
#[derive(Clone)]
enum Op {
    Add,
}

#[derive(Debug)]
#[derive(Clone)]
enum AST {
    Atom(Token),
    Expr(Vec<AST>),
}

fn ast(tokens: Vec<Token>) -> Result<(Vec<AST>, Vec<Token>), &'static str> {
    let (first, rest) = match tokens.split_first() {
        Some((first, rest)) => (first, rest),
        None => return Result::Ok((vec![], vec![])),
    };

    match first {
        Token::RightParen => {
            return Result::Ok((vec![], rest.to_vec()));
        },
        Token::Int(_) | Token::Operator(_) => {
            let (result, rest) = ast(rest.to_vec()).unwrap();
            let mut v = vec![AST::Atom(first.clone())];
            v.extend(result);
            Result::Ok((v, rest))
        },
        Token::LeftParen => {
            let (result, remainder) = ast(rest.to_vec()).unwrap();
            let mut v = vec![AST::Expr(result)];

            let (result, _) = ast(remainder.to_vec()).unwrap();
            v.extend(result);
            Result::Ok((v, vec![]))
        },
        _ =>  return Result::Err("parse error"),
    }
}

fn pretty(v: Vec<AST>, prefix: String) {
    match v.split_first() {
        None => return,
        Some((AST::Atom(t), _)) => {
            println!("{}{:?}", prefix, t);
        },
        Some((AST::Expr(t), _)) => {
            println!("{}Expr:", prefix);
            for token in t {
                pretty(vec![token.clone()], format!("{}{}", prefix, "\t"));
            }
        },
    }
}

fn main() {
   // println!("Hello, world!");
   // let input = "+ 500 700";
   // println!("Got: {:?}", tokenize(&input));

   // let input = "( + 500 700 )";
   // println!("Got: {:?}", tokenize(&input));

   // let input = "100";
   // println!("Got: {:?}", ast(tokenize(&input)));

   // let input = "+ 100 200";
   // println!("Got: {:?}", ast(tokenize(&input)));

   // let input = "(+ 100 200)";
   // println!("Got: {:?}", ast(tokenize(&input)));

   //let input = "(+ 100 (+ 300 400))";
   //println!("Got: {:?}", ast(tokenize(&input)));

 let input = "(+ (+ 300 400) + 100)";
 println!("Got: {:?}", ast(tokenize(&input)));


 let input = "(+ (+ 200 300) (+ 400 500))";
 pretty(ast(tokenize(&input)).unwrap().0, String::new());

 let input = "(+ (+ 200 300) (+ (900 1000) 500))";
 pretty(ast(tokenize(&input)).unwrap().0, String::new());
}//
