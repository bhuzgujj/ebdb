use crate::error::{EbdbError, EbdbResult};
use crate::model::Structure;
use crate::tokenizer::Token;
use std::collections::HashMap;
use std::fmt::format;
use std::str::FromStr;

pub fn parse(tokens: Vec<Token>) -> EbdbResult<HashMap<String, PartialStructure>> {
	let mut partials = HashMap::new();
	let mut sname: Option<String> = None;
	for token in tokens {
		match token {
			Token::Identifier(name) => {
				if sname.is_some() {
					return Err(EbdbError::Custom(format!("Cannot parse multiple name for Structs: {}", name)));
				} else {
					sname = Some(name.clone())
				}
			}
			Token::Curly(curly) => {
				if let Some(n) = sname {
					partials.insert(n, PartialStructure::from_tokens(curly)?);
					sname = None;
				} else {
					return Err(EbdbError::Custom(format!("Structs starts with a name")));
				}
			}
			_ => return Err(EbdbError::Custom(format!("Invalid token for creation of struct: {:?}", token))),
		}
	}
	Ok(partials)
}

pub fn link(partials: HashMap<String, PartialStructure>) -> EbdbResult<Vec<Structure>> {
	let mut structures = Vec::new();

	Ok(structures)
}

pub struct PartialStructure {
	pub fields: HashMap<String, PartialType>,
}

impl PartialStructure {
	fn from_tokens(tokens: Vec<Token>) -> EbdbResult<Self> {
		let mut fields = HashMap::new();
		let mut begining = 0;
		let mut current = begining;
		let mut names: Option<String> = None;
		while current < tokens.len() {
			if names.is_none() {
				if let Token::Identifier(name) = &tokens[current] {
					names = Some(name.clone());
					begining += 1;
				} else {
					return Err(EbdbError::Custom("Name not found for the structs".to_string()));
				}
			} else if let Token::Commas = &tokens[current] {
				fields.insert(names.unwrap().clone(), PartialType::from_tokens(&tokens[begining..current])?);
				names = None;
				begining = current + 1;
			}
			current += 1;
		}
		Ok(PartialStructure { fields })
	}
}

#[derive(Debug, Clone)]
pub struct PartialType {
	types: String,
	size: Option<usize>,
	default: Option<String>,
	amount: usize,
	nullable: bool
}

impl PartialType {
	fn from_tokens(tokens: &[Token]) -> EbdbResult<Self> {
		if let Token::Identifier(types) = &tokens[0] {
			let mut size: Option<usize> = None;
			let mut default: Option<String> = None;
			let mut amount: usize = 1;
			let mut nullable: bool = false;
			for token in &tokens[1..] {
				match token {
					Token::Parenthesis(size_token) => {
						if size_token.len() > 1 {
							return Err(EbdbError::Custom(format!("To many tokens for Parenthesis in types {}", types)));
						} else if size_token.len() == 1 {
							if size.is_some() {
								return Err(EbdbError::Custom(format!("To many Parenthesis in types {}", types)));
							}
							if let Token::Identifier(size_identifier) = &size_token[0] {
								match usize::from_str(size_identifier.as_str()) {
									Ok(value) => {
										size = Some(value)
									}
									Err(_) => {
										return Err(EbdbError::Custom(format!("Parenthisis is not a usize in types {}", types)));
									}
								}
							}
						} else if size.is_none() {
							size = Some(0);
						} else {
							return Err(EbdbError::Custom(format!("Cannot set size twice in types {}", types)));
						}
					},
					Token::Square(amount_token) => {
						if amount_token.len() > 1 {
							return Err(EbdbError::Custom(format!("To many tokens for Square in types {}", types)));
						} else if amount_token.len() == 1 {
							if let Token::Identifier(amount_identifier) = &amount_token[0] {
								match usize::from_str(amount_identifier.as_str()) {
									Ok(value) => {
										amount = value
									}
									Err(_) => {
										return Err(EbdbError::Custom(format!("Square is not a usize in types {}", types)));
									}
								}
							}
						} else {
							amount = 0;
						}
					}
					Token::ExclamationMark => {
						if nullable {
							return Err(EbdbError::Custom(format!("One ExclamationMark maximum in types {}", types)));
						} else {
							// ..
						}
					},
					Token::QuestionMark => {
						if default.is_some() {
							return Err(EbdbError::Custom(format!("One QuestionMark maximum in types {}", types)));
						} else {
							nullable = true;
						}
					},
					Token::Quoted(default_token) => {
						default = Some(default_token.clone())
					},
					_ => return Err(EbdbError::Custom(format!("Invalid token for creation of types {}: {:?}", types, token))),
				}
			}
			Ok(Self {
				types: types.clone(),
				size,
				default,
				amount,
				nullable
			})
		} else {
			Err(EbdbError::Custom("The first token should be an identifier".to_string()))
		}
	}
}