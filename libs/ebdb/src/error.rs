use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::tokenizer::Token;

#[derive(Debug)]
pub enum EbdbError {
	MissingCharacter(char),
	WrongClosingContainer(char, char),
	ClosingNonExistingContainer(char),
	DuplicateFieldName(String),
	ExpectedName(Token),
	Custom(String),
}

impl Display for EbdbError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			EbdbError::MissingCharacter(character) => {
				f.write_str(format!("Could not find the closing character: {}", character).as_str())
			},
			EbdbError::WrongClosingContainer(expected, has) => {
				f.write_str(format!("Expected '{}' and received '{}'", expected, has).as_str())
			},
			EbdbError::ClosingNonExistingContainer(character) => {
				f.write_str(format!("'{}' was closed but never opened", character).as_str())
			},
			EbdbError::DuplicateFieldName(name) => todo!(),
			EbdbError::ExpectedName(token) => todo!(),
			EbdbError::Custom(err) => f.write_str(format!("{}", err).as_str()),
		}
	}
}

impl Error for EbdbError { }

pub type EbdbResult<T> = Result<T, EbdbError>;