//! Parser for the shell command line.
//!
//! Consumes the [`Token`] stream produced by [`crate::tokenize`] and
//! produces an [`AndOr`] tree covering simple commands, redirections,
//! pipelines, and short-circuit `&&`/`||` lists.
//!
//! Out of scope here (handed off to later milestone-0.1 issues): the
//! sequencing operator `;`, backgrounding `&`, and subshell grouping
//! `(` `)`. Their tokens are rejected with
//! [`ParseError::UnsupportedToken`] until the owning issue lands.

use std::error::Error;
use std::fmt;

use crate::token::Token;

/// What kind of file redirection a [`Redirect`] performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectKind {
    /// `< file` — connect `file` to stdin.
    Input,
    /// `> file` — truncate `file` and connect to stdout.
    Output,
    /// `>> file` — append to `file` from stdout.
    Append,
}

/// A single file redirection attached to a simple command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redirect {
    pub kind: RedirectKind,
    pub target: String,
}

/// One stage of a pipeline: a sequence of argument words and the
/// redirections that apply to this stage.
///
/// POSIX permits redirections to appear anywhere in a simple command
/// (`>out echo hi` is legal). We collect them into a separate list
/// rather than preserving the interleaving — execution applies the
/// redirections independently of the argument vector.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SimpleCommand {
    pub words: Vec<String>,
    pub redirects: Vec<Redirect>,
}

/// One or more [`SimpleCommand`]s joined by `|`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pipeline {
    /// Non-empty: a pipeline always has at least one stage.
    pub commands: Vec<SimpleCommand>,
}

/// A left-associative chain of pipelines joined by `&&` / `||`.
///
/// POSIX gives `&&` and `||` equal precedence and left associativity,
/// so `a && b || c` parses as `(a && b) || c`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AndOr {
    Pipeline(Pipeline),
    And(Box<AndOr>, Pipeline),
    Or(Box<AndOr>, Pipeline),
}

/// Reasons parsing can fail.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum ParseError {
    /// Input ended while a construct was still expecting more tokens.
    UnexpectedEof,
    /// A token appeared in a position the grammar doesn't allow.
    UnexpectedToken(&'static str),
    /// A pipeline stage produced no words and no redirections.
    EmptyPipelineStage,
    /// A redirection operator was not followed by a word target.
    MissingRedirectTarget,
    /// A token whose grammar is owned by a later milestone-0.1 issue
    /// (sequencing `;`, background `&`, subshell `(` `)`).
    UnsupportedToken(&'static str),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => f.write_str("unexpected end of input"),
            Self::UnexpectedToken(what) => write!(f, "unexpected token: {what}"),
            Self::EmptyPipelineStage => f.write_str("empty pipeline stage"),
            Self::MissingRedirectTarget => f.write_str("redirection missing target"),
            Self::UnsupportedToken(what) => write!(f, "unsupported token: {what}"),
        }
    }
}

impl Error for ParseError {}

/// Parse a token slice into an [`AndOr`] tree.
///
/// # Errors
///
/// Returns [`ParseError`] when the token stream does not form a valid
/// and-or list under the grammar this issue covers.
pub fn parse(tokens: &[Token]) -> Result<AndOr, ParseError> {
    let mut p = Parser { tokens, pos: 0 };
    let tree = p.parse_and_or()?;
    if p.pos < p.tokens.len() {
        return Err(unsupported_or_unexpected(&p.tokens[p.pos]));
    }
    Ok(tree)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<&'a Token> {
        let t = self.tokens.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn parse_and_or(&mut self) -> Result<AndOr, ParseError> {
        let mut left = AndOr::Pipeline(self.parse_pipeline()?);
        loop {
            match self.peek() {
                Some(Token::AndIf) => {
                    self.bump();
                    let rhs = self.parse_pipeline()?;
                    left = AndOr::And(Box::new(left), rhs);
                }
                Some(Token::OrIf) => {
                    self.bump();
                    let rhs = self.parse_pipeline()?;
                    left = AndOr::Or(Box::new(left), rhs);
                }
                _ => return Ok(left),
            }
        }
    }

    fn parse_pipeline(&mut self) -> Result<Pipeline, ParseError> {
        let mut commands = vec![self.parse_simple_command()?];
        while let Some(Token::Pipe) = self.peek() {
            self.bump();
            commands.push(self.parse_simple_command()?);
        }
        Ok(Pipeline { commands })
    }

    fn parse_simple_command(&mut self) -> Result<SimpleCommand, ParseError> {
        // Discriminate the two empty-stage shapes up front so callers
        // can tell "input ran out" (`a |`) from "operator where a stage
        // was expected" (`| a`).
        match self.peek() {
            None => return Err(ParseError::UnexpectedEof),
            Some(Token::Pipe | Token::AndIf | Token::OrIf) => {
                return Err(ParseError::EmptyPipelineStage);
            }
            _ => {}
        }

        let mut cmd = SimpleCommand::default();
        loop {
            if matches!(
                self.peek(),
                Some(Token::Pipe | Token::AndIf | Token::OrIf) | None,
            ) {
                break;
            }
            match self.bump() {
                Some(Token::Word(w)) => cmd.words.push(w.clone()),
                Some(Token::Less) => cmd
                    .redirects
                    .push(self.finish_redirect(RedirectKind::Input)?),
                Some(Token::Great) => {
                    cmd.redirects
                        .push(self.finish_redirect(RedirectKind::Output)?);
                }
                Some(Token::DGreat) => {
                    cmd.redirects
                        .push(self.finish_redirect(RedirectKind::Append)?);
                }
                Some(other) => return Err(unsupported_or_unexpected(other)),
                None => unreachable!("peeked non-None above"),
            }
        }
        Ok(cmd)
    }

    fn finish_redirect(&mut self, kind: RedirectKind) -> Result<Redirect, ParseError> {
        match self.bump() {
            Some(Token::Word(w)) => Ok(Redirect {
                kind,
                target: w.clone(),
            }),
            _ => Err(ParseError::MissingRedirectTarget),
        }
    }
}

fn unsupported_or_unexpected(tok: &Token) -> ParseError {
    match tok {
        Token::Semi => ParseError::UnsupportedToken(";"),
        Token::Amp => ParseError::UnsupportedToken("&"),
        Token::LParen => ParseError::UnsupportedToken("("),
        Token::RParen => ParseError::UnsupportedToken(")"),
        Token::Pipe => ParseError::UnexpectedToken("|"),
        Token::AndIf => ParseError::UnexpectedToken("&&"),
        Token::OrIf => ParseError::UnexpectedToken("||"),
        Token::Less => ParseError::UnexpectedToken("<"),
        Token::Great => ParseError::UnexpectedToken(">"),
        Token::DGreat => ParseError::UnexpectedToken(">>"),
        Token::Word(_) => ParseError::UnexpectedToken("word"),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{AndOr, ParseError, Pipeline, Redirect, RedirectKind, SimpleCommand, parse};
    use crate::token::tokenize;

    fn parse_str(input: &str) -> Result<AndOr, ParseError> {
        let tokens = tokenize(input).expect("tokenize should succeed in parser tests");
        parse(&tokens)
    }

    fn cmd(words: &[&str]) -> SimpleCommand {
        SimpleCommand {
            words: words.iter().map(|s| (*s).to_string()).collect(),
            redirects: vec![],
        }
    }

    fn pipeline(stages: Vec<SimpleCommand>) -> Pipeline {
        Pipeline { commands: stages }
    }

    #[test]
    fn the_issue_example_parses_to_the_right_tree() {
        // `echo hi | grep h && echo ok`
        // => And(Pipeline[echo hi, grep h], Pipeline[echo ok])
        let got = parse_str("echo hi | grep h && echo ok").unwrap();
        let expected = AndOr::And(
            Box::new(AndOr::Pipeline(pipeline(vec![
                cmd(&["echo", "hi"]),
                cmd(&["grep", "h"]),
            ]))),
            pipeline(vec![cmd(&["echo", "ok"])]),
        );
        assert_eq!(got, expected);
    }

    #[test]
    fn single_simple_command() {
        let got = parse_str("echo hi").unwrap();
        assert_eq!(got, AndOr::Pipeline(pipeline(vec![cmd(&["echo", "hi"])])),);
    }

    #[test]
    fn multi_stage_pipeline() {
        let got = parse_str("a | b | c").unwrap();
        assert_eq!(
            got,
            AndOr::Pipeline(pipeline(vec![cmd(&["a"]), cmd(&["b"]), cmd(&["c"])])),
        );
    }

    #[test]
    fn or_short_circuit_pair() {
        let got = parse_str("a || b").unwrap();
        assert_eq!(
            got,
            AndOr::Or(
                Box::new(AndOr::Pipeline(pipeline(vec![cmd(&["a"])]))),
                pipeline(vec![cmd(&["b"])]),
            ),
        );
    }

    #[test]
    fn and_or_is_left_associative() {
        // a && b || c && d  ==  ((a && b) || c) && d
        let got = parse_str("a && b || c && d").unwrap();
        let a_and_b = AndOr::And(
            Box::new(AndOr::Pipeline(pipeline(vec![cmd(&["a"])]))),
            pipeline(vec![cmd(&["b"])]),
        );
        let or_c = AndOr::Or(Box::new(a_and_b), pipeline(vec![cmd(&["c"])]));
        let expected = AndOr::And(Box::new(or_c), pipeline(vec![cmd(&["d"])]));
        assert_eq!(got, expected);
    }

    #[test]
    fn redirections_attach_to_the_right_stage() {
        let got = parse_str("cat < in | sort > out").unwrap();
        let stage0 = SimpleCommand {
            words: vec!["cat".into()],
            redirects: vec![Redirect {
                kind: RedirectKind::Input,
                target: "in".into(),
            }],
        };
        let stage1 = SimpleCommand {
            words: vec!["sort".into()],
            redirects: vec![Redirect {
                kind: RedirectKind::Output,
                target: "out".into(),
            }],
        };
        assert_eq!(got, AndOr::Pipeline(pipeline(vec![stage0, stage1])));
    }

    #[test]
    fn multiple_redirects_on_one_stage() {
        let got = parse_str("cat <in >out").unwrap();
        let stage = SimpleCommand {
            words: vec!["cat".into()],
            redirects: vec![
                Redirect {
                    kind: RedirectKind::Input,
                    target: "in".into(),
                },
                Redirect {
                    kind: RedirectKind::Output,
                    target: "out".into(),
                },
            ],
        };
        assert_eq!(got, AndOr::Pipeline(pipeline(vec![stage])));
    }

    #[test]
    fn append_redirection() {
        let got = parse_str("echo x >> log").unwrap();
        let stage = SimpleCommand {
            words: vec!["echo".into(), "x".into()],
            redirects: vec![Redirect {
                kind: RedirectKind::Append,
                target: "log".into(),
            }],
        };
        assert_eq!(got, AndOr::Pipeline(pipeline(vec![stage])));
    }

    #[test]
    fn redirection_only_command_is_allowed() {
        let got = parse_str("> file").unwrap();
        let stage = SimpleCommand {
            words: vec![],
            redirects: vec![Redirect {
                kind: RedirectKind::Output,
                target: "file".into(),
            }],
        };
        assert_eq!(got, AndOr::Pipeline(pipeline(vec![stage])));
    }

    #[test]
    fn empty_input_is_unexpected_eof() {
        assert_eq!(parse_str(""), Err(ParseError::UnexpectedEof));
    }

    #[test]
    fn leading_pipe_is_empty_stage() {
        assert_eq!(parse_str("| a"), Err(ParseError::EmptyPipelineStage));
    }

    #[test]
    fn trailing_pipe_is_unexpected_eof() {
        // `a |` consumes `a`, sees `|`, then expects another stage and
        // hits end of input inside `parse_simple_command`.
        assert_eq!(parse_str("a |"), Err(ParseError::UnexpectedEof));
    }

    #[test]
    fn double_pipe_via_separate_tokens_is_empty_stage() {
        // `a | | b` — two `|` tokens, not the `||` operator.
        assert_eq!(parse_str("a | | b"), Err(ParseError::EmptyPipelineStage));
    }

    #[test]
    fn trailing_and_if_is_unexpected_eof() {
        assert_eq!(parse_str("a &&"), Err(ParseError::UnexpectedEof));
    }

    #[test]
    fn leading_and_if_is_empty_stage() {
        assert_eq!(parse_str("&& a"), Err(ParseError::EmptyPipelineStage));
    }

    #[test]
    fn redirect_without_target_is_error() {
        assert_eq!(parse_str("echo >"), Err(ParseError::MissingRedirectTarget));
    }

    #[test]
    fn redirect_followed_by_operator_is_missing_target() {
        assert_eq!(
            parse_str("echo > | cat"),
            Err(ParseError::MissingRedirectTarget),
        );
    }

    #[test]
    fn semicolon_is_unsupported() {
        assert_eq!(parse_str("a ; b"), Err(ParseError::UnsupportedToken(";")));
    }

    #[test]
    fn background_amp_is_unsupported() {
        assert_eq!(parse_str("a &"), Err(ParseError::UnsupportedToken("&")));
    }

    #[test]
    fn parens_are_unsupported() {
        assert_eq!(parse_str("( a )"), Err(ParseError::UnsupportedToken("(")));
    }
}
