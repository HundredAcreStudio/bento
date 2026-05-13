//! Tokenizer for the shell command line.
//!
//! Splits an input string into a stream of typed [`Token`]s. Handles single
//! quotes, double quotes, backslash escapes, and the metacharacters needed
//! by the upcoming parser. `$`-expansion and command substitution are out of
//! scope here — they live in later milestone-0.1 issues.
//!
//! Deliberately deferred: POSIX `\<newline>` line continuation (treated as
//! literal here), IO-number redirections like `2>`, heredoc/here-string
//! recognition (so `<<` produces two `Less` tokens), and position spans.

use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

/// A lexical token produced by [`tokenize`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A word — the unquoted, single-quoted, double-quoted, and
    /// backslash-escaped fragments concatenated together. May be empty:
    /// `""` and `''` each produce a single `Word("")` so callers can
    /// distinguish an explicit empty argument from no argument.
    Word(String),
    /// `|`
    Pipe,
    /// `||`
    OrIf,
    /// `&&`
    AndIf,
    /// `&`
    Amp,
    /// `;`
    Semi,
    /// `<`
    Less,
    /// `>`
    Great,
    /// `>>`
    DGreat,
    /// `(`
    LParen,
    /// `)`
    RParen,
}

/// Reasons tokenization can fail.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum TokenError {
    /// A single-quoted string never closed before end of input.
    UnterminatedSingleQuote,
    /// A double-quoted string never closed before end of input.
    UnterminatedDoubleQuote,
    /// Input ended on a backslash with no character to escape.
    DanglingBackslash,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnterminatedSingleQuote => f.write_str("unterminated single quote"),
            Self::UnterminatedDoubleQuote => f.write_str("unterminated double quote"),
            Self::DanglingBackslash => f.write_str("dangling backslash at end of input"),
        }
    }
}

impl Error for TokenError {}

/// Tokenize a command line into a stream of [`Token`]s.
///
/// # Errors
///
/// Returns [`TokenError`] when a quote is left open or input ends with a
/// dangling backslash.
pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
    let mut tokens = Vec::new();
    let mut current: Option<String> = None;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' | '\n' => flush_word(&mut tokens, &mut current),
            '\'' => read_single_quoted(&mut chars, word_buf(&mut current))?,
            '"' => read_double_quoted(&mut chars, word_buf(&mut current))?,
            '\\' => match chars.next() {
                Some(next) => word_buf(&mut current).push(next),
                None => return Err(TokenError::DanglingBackslash),
            },
            '|' => {
                flush_word(&mut tokens, &mut current);
                if chars.next_if_eq(&'|').is_some() {
                    tokens.push(Token::OrIf);
                } else {
                    tokens.push(Token::Pipe);
                }
            }
            '&' => {
                flush_word(&mut tokens, &mut current);
                if chars.next_if_eq(&'&').is_some() {
                    tokens.push(Token::AndIf);
                } else {
                    tokens.push(Token::Amp);
                }
            }
            '>' => {
                flush_word(&mut tokens, &mut current);
                if chars.next_if_eq(&'>').is_some() {
                    tokens.push(Token::DGreat);
                } else {
                    tokens.push(Token::Great);
                }
            }
            '<' => {
                flush_word(&mut tokens, &mut current);
                tokens.push(Token::Less);
            }
            ';' => {
                flush_word(&mut tokens, &mut current);
                tokens.push(Token::Semi);
            }
            '(' => {
                flush_word(&mut tokens, &mut current);
                tokens.push(Token::LParen);
            }
            ')' => {
                flush_word(&mut tokens, &mut current);
                tokens.push(Token::RParen);
            }
            other => word_buf(&mut current).push(other),
        }
    }

    flush_word(&mut tokens, &mut current);
    Ok(tokens)
}

fn flush_word(tokens: &mut Vec<Token>, current: &mut Option<String>) {
    if let Some(w) = current.take() {
        tokens.push(Token::Word(w));
    }
}

/// Get a mutable reference to the in-progress word buffer, creating an
/// empty one if no word is currently being built. This is what makes
/// `""` and `''` produce empty `Word("")` tokens, and what stitches
/// adjacent quoted/unquoted fragments into a single word.
fn word_buf(current: &mut Option<String>) -> &mut String {
    current.get_or_insert_with(String::new)
}

fn read_single_quoted(chars: &mut Peekable<Chars<'_>>, buf: &mut String) -> Result<(), TokenError> {
    for c in chars.by_ref() {
        if c == '\'' {
            return Ok(());
        }
        buf.push(c);
    }
    Err(TokenError::UnterminatedSingleQuote)
}

fn read_double_quoted(chars: &mut Peekable<Chars<'_>>, buf: &mut String) -> Result<(), TokenError> {
    while let Some(c) = chars.next() {
        match c {
            '"' => return Ok(()),
            '\\' => match chars.peek() {
                Some(&next) if matches!(next, '\\' | '"' | '$' | '`' | '\n') => {
                    buf.push(next);
                    chars.next();
                }
                // POSIX 2.2.3: `\` before any other char inside "..." is literal.
                // Don't consume `next` — the outer loop will handle it.
                Some(_) => buf.push('\\'),
                None => return Err(TokenError::UnterminatedDoubleQuote),
            },
            other => buf.push(other),
        }
    }
    Err(TokenError::UnterminatedDoubleQuote)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::Token::{Amp, AndIf, DGreat, Great, LParen, Less, OrIf, Pipe, RParen, Semi, Word};
    use super::{TokenError, tokenize};

    fn word(s: &str) -> super::Token {
        Word(s.to_string())
    }

    #[test]
    fn empty_input_yields_no_tokens() {
        assert_eq!(tokenize("").unwrap(), vec![]);
        assert_eq!(tokenize("   \t\n  ").unwrap(), vec![]);
    }

    #[test]
    fn splits_words_on_whitespace() {
        assert_eq!(
            tokenize("echo hello world").unwrap(),
            vec![word("echo"), word("hello"), word("world")],
        );
    }

    #[test]
    fn single_quotes_preserve_literally() {
        assert_eq!(
            tokenize("'hello world'").unwrap(),
            vec![word("hello world")]
        );
        assert_eq!(tokenize(r"'$x \n'").unwrap(), vec![word(r"$x \n")]);
    }

    #[test]
    fn double_quotes_group_words() {
        assert_eq!(tokenize("\"a b\"").unwrap(), vec![word("a b")]);
    }

    #[test]
    fn adjacent_fragments_concatenate() {
        assert_eq!(tokenize("a'b'c\"d\"e").unwrap(), vec![word("abcde")]);
    }

    #[test]
    fn backslash_escapes_outside_quotes() {
        assert_eq!(tokenize(r"\$HOME").unwrap(), vec![word("$HOME")]);
        assert_eq!(tokenize(r"a\ b").unwrap(), vec![word("a b")]);
    }

    #[test]
    fn backslash_escapes_inside_double_quotes() {
        assert_eq!(tokenize(r#""\$HOME""#).unwrap(), vec![word("$HOME")]);
        assert_eq!(tokenize(r#""\"""#).unwrap(), vec![word("\"")]);
        assert_eq!(tokenize(r#""\\""#).unwrap(), vec![word("\\")]);
    }

    #[test]
    fn backslash_before_non_escapable_is_literal_in_double_quotes() {
        // POSIX 2.2.3: inside "...", \ is only special before \ " $ ` newline.
        // \n here means backslash-then-n, two characters.
        assert_eq!(tokenize(r#""\n""#).unwrap(), vec![word(r"\n")]);
    }

    #[test]
    fn metacharacters_break_words_without_whitespace() {
        assert_eq!(
            tokenize("echo hi|grep h").unwrap(),
            vec![word("echo"), word("hi"), Pipe, word("grep"), word("h")],
        );
    }

    #[test]
    fn short_circuit_operators() {
        assert_eq!(
            tokenize("a && b || c").unwrap(),
            vec![word("a"), AndIf, word("b"), OrIf, word("c")],
        );
    }

    #[test]
    fn background_and_semicolon() {
        assert_eq!(
            tokenize("a & b ; c").unwrap(),
            vec![word("a"), Amp, word("b"), Semi, word("c")],
        );
    }

    #[test]
    fn redirection_and_grouping_operators() {
        assert_eq!(
            tokenize("> >> < ( )").unwrap(),
            vec![Great, DGreat, Less, LParen, RParen],
        );
    }

    #[test]
    fn empty_quoted_strings_are_words() {
        assert_eq!(tokenize("''").unwrap(), vec![word("")]);
        assert_eq!(tokenize("\"\"").unwrap(), vec![word("")]);
        assert_eq!(
            tokenize("x=''").unwrap(),
            vec![word("x=")],
            "trailing empty single-quote stitches into preceding word",
        );
    }

    #[test]
    fn unterminated_single_quote_is_error() {
        assert_eq!(tokenize("'abc"), Err(TokenError::UnterminatedSingleQuote));
    }

    #[test]
    fn unterminated_double_quote_is_error() {
        assert_eq!(tokenize("\"abc"), Err(TokenError::UnterminatedDoubleQuote));
        assert_eq!(tokenize("\"\\"), Err(TokenError::UnterminatedDoubleQuote));
    }

    #[test]
    fn dangling_backslash_is_error() {
        assert_eq!(tokenize("foo \\"), Err(TokenError::DanglingBackslash));
    }

    #[test]
    fn newline_is_word_separator() {
        assert_eq!(tokenize("a\nb").unwrap(), vec![word("a"), word("b")]);
    }

    #[test]
    fn double_less_does_not_merge() {
        // Heredoc is a later issue; for now `<<` is just two `Less` tokens.
        assert_eq!(tokenize("<<").unwrap(), vec![Less, Less]);
    }

    #[test]
    fn backslash_newline_is_literal_for_now() {
        // POSIX `\<newline>` line continuation is deferred to a later issue;
        // until then this is just a word with an embedded newline.
        assert_eq!(tokenize("a\\\nb").unwrap(), vec![word("a\nb")]);
    }

    #[test]
    fn double_backslash_outside_quotes_is_one_backslash() {
        assert_eq!(tokenize(r"\\").unwrap(), vec![word("\\")]);
    }
}
