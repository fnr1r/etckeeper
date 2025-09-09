use easy_ext::ext;
use thiserror::Error;

use crate::consts::{DOUBLE_QUOTE, SINGLE_QUOTE};

#[derive(Debug, Error)]
pub enum ShellInnerError {
    #[error("bad character in default state")]
    DefaultStateError,
}

#[derive(Debug)]
enum ShellQuoteState {
    Default,
    InDoubleQuoteString,
    InDoubleQuoteStringBackslashed,
    InSingleQuoteString,
}

fn shell_eval_default(
    c: char,
    state: &mut ShellQuoteState,
) -> Result<Option<char>, ShellInnerError> {
    Ok(match c {
        DOUBLE_QUOTE => {
            *state = ShellQuoteState::InDoubleQuoteString;
            None
        }
        SINGLE_QUOTE => {
            *state = ShellQuoteState::InSingleQuoteString;
            None
        }
        _ if c.is_alphanumeric() => Some(c),
        _ => return Err(ShellInnerError::DefaultStateError),
    })
}

fn shell_eval_double_quote(c: char, state: &mut ShellQuoteState) -> Option<char> {
    match c {
        DOUBLE_QUOTE => *state = ShellQuoteState::Default,
        SINGLE_QUOTE => return Some(c),
        '\\' => *state = ShellQuoteState::InDoubleQuoteStringBackslashed,
        _ => todo!(),
    }
    None
}

fn shell_eval_double_quote_backslashed(c: char, state: &mut ShellQuoteState) -> Option<char> {
    *state = ShellQuoteState::InDoubleQuoteString;
    Some(c)
}

fn shell_eval_single_quote(c: char, state: &mut ShellQuoteState) -> Option<char> {
    if c == SINGLE_QUOTE {
        *state = ShellQuoteState::Default;
        None
    } else {
        Some(c)
    }
}

fn shell_eval_inner(
    c: char,
    state: &mut ShellQuoteState,
    res: &mut String,
) -> Result<(), ShellInnerError> {
    use ShellQuoteState as E;
    if let Some(c) = match state {
        E::Default => shell_eval_default(c, state)?,
        E::InDoubleQuoteString => shell_eval_double_quote(c, state),
        E::InDoubleQuoteStringBackslashed => shell_eval_double_quote_backslashed(c, state),
        E::InSingleQuoteString => shell_eval_single_quote(c, state),
    } {
        res.push(c);
    }
    Ok(())
}

#[derive(Debug, Error)]
#[error("shell evaluation failed at index {idx} character '{c}'\n{error}")]
pub struct ShellEvalError {
    idx: usize,
    c: char,
    error: ShellInnerError,
}

#[ext]
impl<T> Result<T, ShellInnerError> {
    fn into_eval_error(self, idx: usize, c: char) -> Result<T, ShellEvalError> {
        self.map_err(|error| ShellEvalError { idx, c, error })
    }
}

pub fn shell_eval(txt: &str) -> Result<String, ShellEvalError> {
    let mut state = ShellQuoteState::Default;
    let mut res = String::new();
    for (idx, c) in txt.chars().enumerate() {
        shell_eval_inner(c, &mut state, &mut res).into_eval_error(idx, c)?;
    }
    Ok(res)
}
