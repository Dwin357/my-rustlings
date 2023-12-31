// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

// Original
/*
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    // fn from_parseint...
}
*/

// Option.1 -> based on the observation that the result-chaining methods (ie map, or_else, etc)
//// all seem to have build in logic to only process `Ok`s or `Err`s, what if I have parse -> map
//// w/ map_err fn at the end which takes in a generic and maps it to its wrapper
// => *sad panda* turns out you are not allowed to overload a method with different arities in Rust
/*
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    // fn from_parseint...
}
*/

// Option.2 -> lets just be ham-fisted to try and get something out there
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }    
}


fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.

    // Original
//    let x: i64 = s.parse().unwrap();
//    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)

    // Option.1
//    s.parse()
//     .map(|x| PositiveNonzeroInteger::new(x))
//     .map_err(ParsePosNonzeroError);

    // Option.2
    /*
    s.parse::<i64>()
     .map_err(|e| ParsePosNonzeroError::from_parseint(e))
     .map(|n| PositiveNonzeroInteger::new(n))
     .map_err(|e| ParsePosNonzeroError::from_creation(e))
     .unwrap() // I don't know why this needs to be here???
*/

    // Option.3 -> fuck it, looked up the ans :: https://github.com/rust-lang/rustlings/issues/808
    //// ok, so...
    // 1) I thought you couldn't use `?` on something with a return
    // 2) not a huge fan of all the stuff inside the Ok, but *shrug* I'm done with this.
    // 3) fml - looks like that chaining was a bad path    
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    Ok(PositiveNonzeroInteger::new(x).map_err(|err| ParsePosNonzeroError::from_creation(err))?)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

   // #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

  //  #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

 //   #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

 //   #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
