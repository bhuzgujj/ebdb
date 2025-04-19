use crate::error::{EbdbError, EbdbResult};
use crate::error::EbdbError::{ClosingNonExistingContainer, WrongClosingContainer};

#[derive(Debug, Clone)]
pub enum Token {
	Identifier(String),

	Parenthesis(Vec<Token>),
	Curly(Vec<Token>),
	Square(Vec<Token>),
	Quoted(String),

	Commas,
	QuestionMark,
	ExclamationMark,
}

pub struct Tokenizer<'tokenizer> {
	tokens: Vec<Token>,
	string: &'tokenizer str,
	back_index: Option<usize>,
	exit_character: Option<char>,
}

impl<'tokenizer> Tokenizer<'tokenizer> {
	pub fn new(string: &'tokenizer str) -> Self {
		Self {
			tokens: Vec::new(),
			string,
			back_index: None,
			exit_character: None,
		}
	}

	fn from_slice(string: &'tokenizer str, character: char) -> Self {
		Self {
			tokens: Vec::new(),
			string,
			back_index: None,
			exit_character: Some(character),
		}
	}
	pub fn parse(mut self) -> EbdbResult<Vec<Token>> {
		let (token, _) = self.internal_parse()?;
		Ok(token)
	}

	fn internal_parse(mut self) -> EbdbResult<(Vec<Token>, usize)> {
		let mut current_index: usize = 0;
		let characters = self.string.as_bytes();
		let end_index: usize = self.string.len();
		while current_index < end_index {
			let character = characters[current_index] as char;
			if self.exit_character == Some(character) {
				if let Some(back_index) = self.back_index {
					let slice = String::from(&self.string[back_index..current_index]);
					self.tokens.push(Token::Identifier(slice));
				}
				return Ok((self.tokens, current_index));
			}
			match character {
				' ' | '\t' | '\n' | '\r' => {
					if let Some(back_index) = self.back_index {
						let slice = String::from(&self.string[back_index..current_index]);
						self.tokens.push(Token::Identifier(slice));
						self.back_index = None;
					}
				},
				'(' => {
					if let Some(back_index) = self.back_index {
						let slice = String::from(&self.string[back_index..current_index]);
						self.tokens.push(Token::Identifier(slice));
						self.back_index = None;
					}
					let (tokens, closing_index) = Tokenizer::from_slice(&self.string[current_index + 1..], ')').internal_parse()?;
					self.tokens.push(Token::Parenthesis(tokens));
					self.back_index = None;
					current_index += closing_index + 1;
				}
				'{' => {
					if let Some(back_index) = self.back_index {
						let slice = String::from(&self.string[back_index..current_index]);
						self.tokens.push(Token::Identifier(slice));
						self.back_index = None;
					}
					let (tokens, closing_index) = Tokenizer::from_slice(&self.string[current_index + 1..], '}').internal_parse()?;
					self.tokens.push(Token::Curly(tokens));
					self.back_index = None;
					current_index += closing_index + 1;
				}
				'[' => {
					if let Some(back_index) = self.back_index {
						let slice = String::from(&self.string[back_index..current_index]);
						self.tokens.push(Token::Identifier(slice));
						self.back_index = None;
					}
					let (tokens, closing_index) = Tokenizer::from_slice(&self.string[current_index + 1..], ']').internal_parse()?;
					self.tokens.push(Token::Square(tokens));
					self.back_index = None;
					current_index += closing_index + 1;
				}
				')' | '}' | ']' => {
					return Err(if let Some(expected) = self.exit_character {
						WrongClosingContainer(expected, character)
					} else {
						ClosingNonExistingContainer(character)
					})
				}
				',' => {
					if let Some(back_index) = self.back_index {
						let slice = String::from(&self.string[back_index..current_index]);
						self.tokens.push(Token::Identifier(slice));
						self.back_index = None;
					}

					self.tokens.push(Token::Commas);
				}
				'!' => {
					if let Some(back_index) = self.back_index {
						let slice = String::from(&self.string[back_index..current_index]);
						self.tokens.push(Token::Identifier(slice));
						self.back_index = None;
					}

					self.tokens.push(Token::ExclamationMark);
				}
				'?' => {
					if let Some(back_index) = self.back_index {
						let slice = String::from(&self.string[back_index..current_index]);
						self.tokens.push(Token::Identifier(slice));
						self.back_index = None;
					}

					self.tokens.push(Token::QuestionMark);
				}
				_ => {
					if self.back_index.is_none() {
						self.back_index = Some(current_index);
					}
				}
			}
			current_index += 1;
		}

		if let Some(back_index) = self.back_index {
			let slice = String::from(&self.string[back_index..]);
			self.tokens.push(Token::Identifier(slice));
		}

		if let Some(character) = self.exit_character {
			Err(EbdbError::MissingCharacter(character))
		} else {
			Ok((self.tokens, self.string.len()))
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::tokenizer::Tokenizer;

	#[test]
	fn test_parse() {
		let tokenizer = Tokenizer::new("hello world(23) ").parse().unwrap();
	}
}