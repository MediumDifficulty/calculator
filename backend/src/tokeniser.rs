pub fn tokenise(input: String) -> Result<Vec<Token>, String> {
    let mut iter = input.chars();
    let mut tokens: Vec<Token> = Vec::new();

    let mut number_buffer = String::new();

    loop {
        let c = iter.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();

        if c.is_numeric() || c == '.' {
            number_buffer.push(c);
            continue;
        }

        if !number_buffer.is_empty() {
            tokens.push(Token::Number(match number_buffer.parse::<f64>() {
                Ok(n) => n,
                Err(_) => return Err(format!("Failed to parse number: {}", number_buffer).to_string()),
            }));
            number_buffer.clear();
        }

        tokens.push(match Token::from_single(c) {
            Ok(token) => token,
            Err(e) => return Err(e),
        })
    }

    if !number_buffer.is_empty() {
        tokens.push(Token::Number(match number_buffer.parse::<f64>() {
            Ok(n) => n,
            Err(_) => return Err(format!("Failed to parse number: {}", number_buffer).to_string()),
        }));
        number_buffer.clear();
    }

    Ok(tokens)
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Raise,
    OpenBracket,
    CloseBracket,
}

impl Token {
    pub fn from_single(c: char) -> Result<Self, String> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Subtract),
            '*' => Ok(Self::Multiply),
            '/' => Ok(Self::Divide),
            '^' => Ok(Self::Raise),
            '(' => Ok(Self::OpenBracket),
            ')' => Ok(Self::CloseBracket),
            _ => Err(format!("Invalid token: {}", c).to_string())
        }
    }
}