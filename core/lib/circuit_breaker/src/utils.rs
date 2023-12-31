// Importing Token from zksync_types::ethabi module.
use zksync_types::ethabi::Token;

// Function to unwrap a Tuple Token into a vector of Tokens.
pub fn unwrap_tuple(token: Token) -> Vec<Token> {
    // Checking if the provided Token is a Tuple.
    if let Token::Tuple(tokens) = token {
        // If it's a Tuple, return its inner tokens.
        tokens
    } else {
        // If it's not a Tuple, panic and display the unexpected token.
        panic!("Tuple was expected, got: {}", token);
    }
}
