use regex::Regex;

/// Defines a list of tokens that can be streamed in a queue
pub struct TokenList
{
    /// Defines the queue list of tokens
    pub tokens: Vec<Token>,

    /// Defines the current queue index of the tokens
    ind: usize,
}

impl TokenList
{
    /// Returns a new TokenList with the tokens provided
    pub fn new(tokens: Vec<Token>) -> TokenList
    {
        return TokenList
        {
            tokens: tokens,
            ind: 0
        };
    }

    /// Resets the queue location to the start
    pub fn reset(&mut self)
    {
        self.ind = 0;
    }

    /// Returns true if another token is available in the queue to read
    pub fn available(&self) -> bool
    {
        return self.ind < self.tokens.len();
    }

    /// Returns a Token if available; otherwise returns None
    pub fn peek(&self) -> Option<Token>
    {
        return if self.available()
        {
            Some(self.tokens[self.ind].clone())
        }
        else
        {
            None
        };
    }

    /// Returns a Token if available and moves to the next token in the queue; otherwise None
    pub fn pop(&mut self) -> Option<Token>
    {
        return if self.available()
        {
            let val = self.peek();
            self.ind += 1;
            val
        }
        else
        {
            None
        };
    }
}

/// Defines a token type that stores information associated with a particular token
#[derive(PartialEq, Clone)]
pub enum Token
{
    Int(i32),
    Float(f32),
    Bool(bool),
    Keyword(String),
    Operator(String),
    Variable(String)
}

impl std::fmt::Debug for Token
{
    /// Returns the debug structure representation of the current token
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let mut debug_struct = f.debug_struct("Token");

        match self
        {
            Token::Int(v) => debug_struct.field("Int", &v),
            Token::Float(v) => debug_struct.field("Float", &v),
            Token::Bool(v) => debug_struct.field("Bool", &v),
            Token::Keyword(v) => debug_struct.field("Keyword", &v),
            Token::Operator(v) => debug_struct.field("Operator", &v),
            Token::Variable(v) => debug_struct.field("Variable", &v)
        };

        return debug_struct.finish();
    }
}

impl ToString for Token
{
    /// Converts the token into a string representation
    fn to_string(&self) -> String
    {
        return match self
        {
            Token::Int(v) => format!("(int {0:})", *v),
            Token::Float(v) => format!("(float {0:})", *v),
            Token::Bool(v) => format!("(bool {0:})", if *v { "true" } else { "false" }),
            Token::Keyword(v) => format!("(keyword {0:})", v),
            Token::Operator(v) => format!("(operator '{0:}')", v),
            Token::Variable(v) => format!("(variable {0:})", v),
        }
    }
}

impl Token
{
    /// Defines the allowed operators list for the ZBasic language
    const OPERATORS: [&'static str; 16] = [
        ";",
        ",",
        "(",
        ")",
        "{",
        "}",
        "+",
        "*",
        "-",
        "/",
        "=",
        "&&",
        "||",
        "==",
        "!=",
        "!"
    ];

    /// Defines the keywords provided by the ZBasic language
    const KEYWORDS: [&'static str; 3] = [
        "if",
        "else",
        "for"
    ];

    /// Splits a string into a token list, separating by operators and whitespace, to return
    /// a vector of strings representing individiual tokens
    fn split_tokens(input: &str) -> Vec<String>
    {
        // Define the resulting string list and the current word tracker
        let mut string_list: Vec<String> = Vec::new();
        let mut current: String = String::new();

        // Define a function to check for separation
        // Returns the length of characters to read in as the next operator
        fn check_separator(input: &str) -> usize
        {
            // Define the maximum length of the operators
            let max_len = Token::OPERATORS.iter().map(|v| v.len()).max().unwrap();

            // Define a vector for possible values
            let mut possible = Token::OPERATORS.iter().map(|v| v.len() <= input.len()).collect::<Vec<bool>>();
            let mut num_possible = possible.iter().filter(|v| **v).count();

            // Iterate over values
            for size in 1..(std::cmp::min(max_len, input.len()) + 1)
            {
                // Extract the current string
                let val = &input[..size];

                // Loop through all possibilities
                for i in 0..possible.len()
                {
                    if !possible[i]
                    {
                        continue;
                    }

                    let op = Token::OPERATORS[i];

                    if op.len() == val.len() && op != val
                    {
                        possible[i] = false;
                        num_possible -= 1;
                    }
                }
            }

            // Check remaining possibilities and see if one is a subset of another
            if num_possible > 1
            {
                // Loop over each possibility
                for i in 0..possible.len()
                {
                    // Skip any that are not possible
                    if !possible[i]
                    {
                        continue;
                    }

                    // Loop over each future possibility
                    for j in (i+1)..possible.len()
                    {
                        // Skip any that are not possible
                        if !possible[j]
                        {
                            continue;
                        }

                        // Extract the two strings
                        let pi = Token::OPERATORS[i];
                        let pj = Token::OPERATORS[j];

                        // Determine which value is smaller/larger
                        let larger;
                        let smaller;
                        let smaller_ind;

                        if pi.len() > pj.len()
                        {
                            larger = pi;
                            smaller = pj;
                            smaller_ind = j;
                        }
                        else
                        {
                            larger = pj;
                            smaller = pi;
                            smaller_ind = i;
                        }

                        // Check if one is a subset of the other, and remove if so
                        if smaller == &larger[..smaller.len()]
                        {
                            possible[smaller_ind] = false;
                            num_possible -= 1;
                        }
                    }
                }
            }

            // Check resulting values
            if num_possible == 0
            {
                return 0;
            }
            else if num_possible == 1
            {
                for i in 0..possible.len()
                {
                    if possible[i]
                    {
                        return Token::OPERATORS[i].len();
                    }
                }
                panic!();
            }
            else
            {
                panic!();
            }
        }

        // Define flags for operator flags
        let mut token_found = true;
        let mut token_len = 0usize;

        // Define the character values
        let input_chars = input.chars().collect::<Vec<char>>();

        // Loop through each character in the input
        let mut i = 0usize;
        while i < input_chars.len()
        {
            // Extract the current character
            let c = input_chars[i];

            // Check for a token match
            if token_found && token_len > 0
            {
                current.push(c);
                token_len -= 1;
                i += 1;
            }
            else
            {
                if token_found
                {
                    if !current.is_empty()
                    {
                        string_list.push(current);
                        current = String::new();
                    }
                    token_found = false;
                }

                let token_test_len = check_separator(&input[i..]);

                if token_test_len > 0
                {
                    token_found = true;
                    token_len = token_test_len;

                    if !current.is_empty()
                    {
                        string_list.push(current);
                        current = String::new();
                    }
                }
                else if c.is_whitespace()
                {
                    if !current.is_empty()
                    {
                        string_list.push(current);
                        current = String::new();
                    }
                    i += 1;
                }
                else
                {
                    current.push(c);
                    i += 1;
                }
            }

        }

        // Add any last word if not empty
        if !current.is_empty()
        {
            string_list.push(current);
        }

        // Return the list
        return string_list;
    }

    /// Tokenizes a string to extract the string into a list of tokens if possible. If an invalid
    /// token is identified, an error string will be returned
    pub fn tokenize(input: &str) -> Result<TokenList, String>
    {
        // Split the input string into words
        let words = Token::split_tokens(input);

        // Define the token list
        let mut tokens: Vec<Token> = Vec::new();

        fn check_for_token(word: &str) -> Result<Token, String>
        {
            // Check for keyword
            for k in Token::KEYWORDS.iter()
            {
                if word == *k
                {
                    return Ok(Token::Keyword(word.to_string()));
                }
            }

            // Check for boolean
            if word == "true" || word == "false"
            {
                return Ok(Token::Bool(word == "true"));
            }

            // Define numeric regex expressions
            let float_re = Regex::new(r"^-?(([0-9]+\.[0-9]*)|([0-9]*\.[0-9]+))$").unwrap();
            let int_re = Regex::new(r"^-?([0-9]+)$").unwrap();

            // Check for a match
            if float_re.is_match(word)
            {
                match word.parse::<f32>()
                {
                    Ok(v) => return Ok(Token::Float(v)),
                    Err(_) => return Err(format!("unable to parse token {0:} as float", word))
                }
            }

            if int_re.is_match(word)
            {
                match word.parse::<i32>()
                {
                    Ok(v) => return Ok(Token::Int(v)),
                    Err(_) => return Err(format!("unable to parse token {0:} as int", word))
                }
            }

            // Check for operator
            for op in Token::OPERATORS.iter()
            {
                if word == *op
                {
                    return Ok(Token::Operator(word.to_string()));
                }
            }

            // Check for variable
            let var_re = Regex::new(r"^([a-zA-Z])([a-zA-Z0-9])*$").unwrap();

            if var_re.is_match(word)
            {
                return Ok(Token::Variable(word.to_string()));
            }

            return Err(format!("unable to parse {0:} as a token", word));
        }

        // Iterate over each word type
        for w in words.iter()
        {
            // Check for a valid result
            match check_for_token(w)
            {
                Ok(v) => tokens.push(v),
                Err(s) => return Err(s)
            }
        }

        return Ok(TokenList::new(tokens));
    }
}

/// Defines tests of the Token types
#[cfg(test)]
mod tests
{
    use super::{Token, TokenList};

    /// Tests the split works/tokens class
    #[test]
    fn split_tokens() -> ()
    {
        // Define the words and expected words
        let words = Token::split_tokens("this   is(a test&&=||)====998 -13 -2.0");
        let expected_words = vec!{
            "this",
            "is",
            "(",
            "a",
            "test",
            "&&",
            "=",
            "||",
            ")",
            "==",
            "==",
            "998",
            "-",
            "13",
            "-",
            "2.0"
        };

        // Ensure that lengths match
        assert_eq!(words.len(), expected_words.len());

        // Ensure that values match
        for i in 0..words.len()
        {
            assert_eq!(words[i], expected_words[i]);
        }
    }

    /// Tests the tokenize function against valid, expected input
    #[test]
    fn tokenize_valid()
    {
        // Define the valid test input
        let token_valid_input= "this   is(a+test&&=||)====998 -13 a93 Z384=3.458 for if FOR If else !test != !asdf";

        // Define the expected test output
        let tokens_expected: Vec<Token> = vec!{
            Token::Variable(String::from("this")),
            Token::Variable(String::from("is")),
            Token::Operator(String::from("(")),
            Token::Variable(String::from("a")),
            Token::Operator(String::from("+")),
            Token::Variable(String::from("test")),
            Token::Operator(String::from("&&")),
            Token::Operator(String::from("=")),
            Token::Operator(String::from("||")),
            Token::Operator(String::from(")")),
            Token::Operator(String::from("==")),
            Token::Operator(String::from("==")),
            Token::Int(998),
            Token::Operator(String::from("-")),
            Token::Int(13),
            Token::Variable(String::from("a93")),
            Token::Variable(String::from("Z384")),
            Token::Operator(String::from("=")),
            Token::Float(3.458),
            Token::Keyword(String::from("for")),
            Token::Keyword(String::from("if")),
            Token::Variable(String::from("FOR")),
            Token::Variable(String::from("If")),
            Token::Keyword(String::from("else")),
            Token::Operator(String::from("!")),
            Token::Variable(String::from("test")),
            Token::Operator(String::from("!=")),
            Token::Operator(String::from("!")),
            Token::Variable(String::from("asdf"))
        };

        let tokens_result = Token::tokenize(token_valid_input);

        assert_eq!(tokens_result.is_ok(), true);

        let mut token_list: TokenList = tokens_result.unwrap();
        let tokens = &token_list.tokens;

        assert_eq!(tokens.len(), tokens_expected.len());

        for i in 0..tokens.len()
        {
            assert_eq!(tokens[i], tokens_expected[i]);
        }

        // Check that the token list provides the correct outputs
        let mut curr_ind = 0usize;
        while token_list.available()
        {
            let t = token_list.pop().unwrap();
            assert_eq!(t, tokens_expected[curr_ind]);
            curr_ind += 1;
        }

        // Ensure that the token list is now empty
        assert_eq!(token_list.available(), false);
        assert_eq!(token_list.peek(), None);
        assert_eq!(token_list.pop(), None);

        // Reset the token list and check that the first two tokens are correct
        token_list.reset();

        assert_eq!(token_list.available(), true);
        assert_eq!(token_list.pop().unwrap(), tokens_expected[0]);
        assert_eq!(token_list.pop().unwrap(), tokens_expected[1]);
    }

    /// Tests the tokenize input against invalid input
    #[test]
    fn tokenize_invalid()
    {
        assert_eq!(Token::tokenize("123.345.3").is_err(), true);
        assert_eq!(Token::tokenize("234a34").is_err(), true);
        assert_eq!(Token::tokenize("[asdf]").is_err(), true);
    }
}
