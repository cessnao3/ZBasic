use token::Token;

mod token;
mod lexer;
mod variable;

fn main()
{
    let q = Token::Variable("Hello".to_string());
    println!("{0:}", q.to_string());

    match Token::tokenize("this   is(atest&&=||)====998 a93 Z384=3.458484")
    {
        Ok(mut tokens) =>
        {
            while tokens.available()
            {
                println!("{0:}", tokens.pop().unwrap().to_string());
            }
        },
        Err(e) => println!("Error: {0:}", e)
    }
}
