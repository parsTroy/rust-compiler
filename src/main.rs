mod ast;

fn main() {
    println!("Simple Input:");

    let input = "10";

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vac::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("{:?}", tokens);
}
