//! Lesson 2.3.2 — Result and the ? operator.
//!
//! Contract: `contracts/c1-l232-result-question-v1.yaml`
//! Transcript anchor: "recoverable errors in Rust use a type called Result"
//!
//! The demo proves one equivalence: `chain_question` (using ?) produces
//! byte-identical output to `chain_match` (using nested match) for every
//! input in the tested sample space. This is the provable equivalence
//! between the ? operator and its desugared match form.

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ParseAgeError {
    #[error("not a number: {0:?}")]
    NotANumber(String),
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DivisionError {
    #[error("divide by zero")]
    DivideByZero,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AppError {
    #[error(transparent)]
    Parse(#[from] ParseAgeError),
    #[error(transparent)]
    Math(#[from] DivisionError),
}

/// CONTRACT: parse_age
/// Invariant: Err(NotANumber(_)) preserves the offending input verbatim.
pub fn parse_age(s: &str) -> Result<u32, ParseAgeError> {
    s.parse::<u32>()
        .map_err(|_| ParseAgeError::NotANumber(s.to_owned()))
}

/// CONTRACT: divide
/// Invariant: Err(DivideByZero) iff d == 0; Ok(n / d) otherwise.
/// Uses `checked_div` — the idiomatic Option-returning primitive — then lifts
/// to Result via `ok_or`.
pub fn divide(n: u32, d: u32) -> Result<u32, DivisionError> {
    n.checked_div(d).ok_or(DivisionError::DivideByZero)
}

/// CONTRACT: chain_question
/// Uses the ? operator with From-conversion to propagate both error types.
pub fn chain_question(s: &str, d: u32) -> Result<u32, AppError> {
    let n = parse_age(s)?;
    let q = divide(n, d)?;
    Ok(q)
}

/// CONTRACT: chain_match
/// Same logic as chain_question, but using explicit nested match. This exists
/// solely as an ORACLE for proving equivalence with the ? version.
pub fn chain_match(s: &str, d: u32) -> Result<u32, AppError> {
    match parse_age(s) {
        Ok(n) => match divide(n, d) {
            Ok(q) => Ok(q),
            Err(e) => Err(AppError::Math(e)),
        },
        Err(e) => Err(AppError::Parse(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_age_ok() {
        assert_eq!(parse_age("42"), Ok(42));
    }

    #[test]
    fn parse_age_rejects_non_numeric() {
        assert_eq!(
            parse_age("abc"),
            Err(ParseAgeError::NotANumber("abc".into()))
        );
    }

    #[test]
    fn parse_age_rejects_empty() {
        assert_eq!(parse_age(""), Err(ParseAgeError::NotANumber(String::new())));
    }

    #[test]
    fn parse_age_rejects_negative() {
        // u32 parse rejects "-1".
        assert_eq!(parse_age("-1"), Err(ParseAgeError::NotANumber("-1".into())));
    }

    #[test]
    fn divide_ok() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn divide_by_zero() {
        assert_eq!(divide(10, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn chain_question_ok() {
        assert_eq!(chain_question("42", 2), Ok(21));
    }

    #[test]
    fn chain_question_propagates_parse_error() {
        let expected = Err(AppError::Parse(ParseAgeError::NotANumber("abc".into())));
        assert_eq!(chain_question("abc", 2), expected);
    }

    #[test]
    fn chain_question_propagates_division_error() {
        let expected = Err(AppError::Math(DivisionError::DivideByZero));
        assert_eq!(chain_question("10", 0), expected);
    }

    #[test]
    fn parse_error_display_preserves_input() {
        let e = ParseAgeError::NotANumber("oops".into());
        assert_eq!(format!("{e}"), "not a number: \"oops\"");
    }

    #[test]
    fn division_error_display_is_stable() {
        let e = DivisionError::DivideByZero;
        assert_eq!(format!("{e}"), "divide by zero");
    }

    #[test]
    fn app_error_display_is_transparent() {
        // #[error(transparent)] forwards Display to the inner error.
        let parse: AppError = ParseAgeError::NotANumber("bad".into()).into();
        assert_eq!(format!("{parse}"), "not a number: \"bad\"");

        let math: AppError = DivisionError::DivideByZero.into();
        assert_eq!(format!("{math}"), "divide by zero");
    }

    #[test]
    fn app_error_debug_prints_variants() {
        let parse: AppError = ParseAgeError::NotANumber("bad".into()).into();
        assert!(format!("{parse:?}").contains("Parse"));
        let math: AppError = DivisionError::DivideByZero.into();
        assert!(format!("{math:?}").contains("Math"));
    }

    /// CONTRACT: chain_equivalence (oracle test)
    /// For every input in the sample space, chain_question and chain_match
    /// must produce identical output. This is the provable equivalence of
    /// the ? operator and its desugared match form.
    #[test]
    fn chain_equivalence_oracle() {
        let inputs = ["42", "0", "1", "abc", "", "-1", "100", "7"];
        let divisors = [0u32, 1, 2, 3, 7, 10, 100];
        for s in inputs {
            for d in divisors {
                let q = chain_question(s, d);
                let m = chain_match(s, d);
                assert_eq!(q, m, "divergence at input=({s:?}, {d})");
            }
        }
    }
}
