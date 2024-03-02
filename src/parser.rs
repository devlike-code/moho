use pest::{
    error::LineColLocation,
    iterators::{Pair, Pairs},
    Parser, Span,
};

use crate::ast::{MohoAST, Rule};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Default,
    Nullptr,
    Char(u8),
    Bool(bool),
    Float(f64),
    Str(String),
    Integer(isize),
}

#[derive(Debug, Clone)]
pub enum Type {
    None,
    Char,
    Bool,
    Float,
    String,
    Integer,
    Array(Box<Type>),
    Matrix(Box<Type>),
    Pointer(Box<Type>),
    Reference(Box<Type>),
    Class(String),
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: Option<Value>,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Block(Block),
    Field(Field),
}

#[derive(Debug, Clone)]
pub struct Field {
    pub properties: Vec<Property>,
    pub name: String,
    pub typ: Type,
    pub value: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub properties: Vec<Property>,
    pub inner: Vec<Declaration>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub inherit: Vec<String>,
    pub inner: Block,
}

#[derive(Debug, Clone)]
pub struct TranslationUnit(pub Vec<Class>);

#[derive(Debug)]
pub struct MohoParser;

#[derive(Debug, Clone)]
pub enum MohoError<'a> {
    TokenizerError(String, LineColLocation),
    ParsePropertyError(String, Span<'a>),
    ParseValueError(String, Span<'a>),
    ParseClassError(String, Span<'a>),
    DeclarationError(String, Span<'a>),
    ParseTypeError(String, Span<'a>),
    ParseFieldError(String, Span<'a>),
}

impl MohoParser {
    pub fn apply(input: &str) -> Result<TranslationUnit, MohoError> {
        let mut result = vec![];
        match MohoAST::parse(Rule::moho, input) {
            Ok(parsed) => {
                for pair in parsed {
                    for class in pair.into_inner() {
                        result.push(MohoParser::parse_class(class)?);
                    }
                }

                Ok(TranslationUnit(result))
            }
            Err(err) => Err(MohoError::TokenizerError(err.to_string(), err.line_col)),
        }
    }

    fn parse_class(class: Pair<'_, Rule>) -> Result<Class, MohoError> {
        assert_eq!(class.as_rule(), Rule::class);

        let mut name = "";
        let mut inherit = vec![];

        let mut result = Block {
            properties: vec![],
            inner: vec![],
        };

        let span = class.as_span();
        for pair in class.into_inner() {
            if matches!(pair.as_rule(), Rule::properties) {
                result
                    .properties
                    .extend(MohoParser::parse_properties(pair)?);
            } else if matches!(pair.as_rule(), Rule::name) {
                name = pair.as_str();
            } else if matches!(pair.as_rule(), Rule::inheritance) {
                inherit = pair
                    .as_str()
                    .split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect()
            } else if matches!(pair.as_rule(), Rule::declaration) {
                result.inner.push(Self::parse_declaration(pair)?);
            } else {
                return Err(MohoError::ParseClassError(
                    format!("Cannot parse {:?} in class.", pair),
                    span,
                ));
            }
        }

        if name.is_empty() {
            return Err(MohoError::ParseClassError(
                "Class name missing.".into(),
                span,
            ));
        }

        Ok(Class {
            name: name.to_string(),
            inherit,
            inner: result,
        })
    }

    fn parse_properties(pairs: Pair<'_, Rule>) -> Result<Vec<Property>, MohoError> {
        pairs.into_inner().map(Self::parse_property).collect()
    }

    fn parse_property(prop: Pair<'_, Rule>) -> Result<Property, MohoError> {
        let span = prop.as_span();

        let mut iter = prop.into_inner();
        let Some(name) = iter.next() else {
            return Err(MohoError::ParsePropertyError(
                "Expected property name".into(),
                span,
            ));
        };

        let value = Self::parse_value(&mut iter)?;

        Ok(Property {
            name: name.as_str().to_string(),
            value: if value == Value::Default {
                None
            } else {
                Some(value)
            },
        })
    }

    fn match_value(val: Pair<'_, Rule>) -> Result<Value, MohoError> {
        match val.as_rule() {
            Rule::bool => {
                Ok(Value::Bool(val.as_str().parse::<bool>().map_err(|e| {
                    MohoError::ParseValueError(e.to_string(), val.as_span())
                })?))
            }

            Rule::char => {
                Ok(Value::Char(val.as_str().parse::<u8>().map_err(|e| {
                    MohoError::ParseValueError(e.to_string(), val.as_span())
                })?))
            }

            Rule::string => Ok(Value::Str(val.as_str().to_string())),

            Rule::integer => {
                Ok(Value::Integer(val.as_str().parse::<isize>().map_err(
                    |e| MohoError::ParseValueError(e.to_string(), val.as_span()),
                )?))
            }

            Rule::float => {
                Ok(Value::Float(val.as_str().parse::<f64>().map_err(|e| {
                    MohoError::ParseValueError(e.to_string(), val.as_span())
                })?))
            }

            Rule::value => {
                let span = val.as_span();
                if let Some(val) = val.into_inner().next() {
                    Self::match_value(val)
                } else {
                    return Err(MohoError::ParsePropertyError(
                        "Failed to parse property".into(),
                        span,
                    ));
                }
            }
            _ => Err(MohoError::ParseValueError(
                "Cannot read value".into(),
                val.as_span(),
            )),
        }
    }

    fn parse_value<'a>(iter: &mut Pairs<'a, Rule>) -> Result<Value, MohoError<'a>> {
        if let Some(value) = iter.next() {
            Self::match_value(value)
        } else {
            Ok(Value::Default)
        }
    }

    fn parse_declaration(decl: Pair<'_, Rule>) -> Result<Declaration, MohoError> {
        let span = decl.as_span();
        let mut result = Block {
            properties: vec![],
            inner: vec![],
        };

        for pair in decl.into_inner() {
            if matches!(pair.as_rule(), Rule::properties) {
                result
                    .properties
                    .extend(MohoParser::parse_properties(pair)?);
            } else if matches!(pair.as_rule(), Rule::field_decl) {
                result.inner.push(Self::parse_field(pair)?);
            } else if matches!(pair.as_rule(), Rule::block_decl) {
                for inner in pair.into_inner() {
                    result.inner.push(Self::parse_declaration(inner)?);
                }
            } else {
                return Err(MohoError::DeclarationError(
                    format!("Unexpected token {:?} in block.", pair),
                    span,
                ));
            }
        }

        Ok(Declaration::Block(result))
    }

    fn parse_field(field: Pair<'_, Rule>) -> Result<Declaration, MohoError> {
        let span = field.as_span();
        let mut result = Field {
            properties: vec![],
            name: "".into(),
            typ: Type::None,
            value: None,
        };

        for pair in field.into_inner() {
            if matches!(pair.as_rule(), Rule::properties) {
                result
                    .properties
                    .extend(MohoParser::parse_properties(pair)?);
            } else if matches!(pair.as_rule(), Rule::type_decl) {
                result.typ = Self::parse_type(pair)?;
            } else if matches!(pair.as_rule(), Rule::identifier) {
                result.name = pair.as_str().trim().to_string();
            } else if matches!(pair.as_rule(), Rule::value) {
                result.value = Some(Self::parse_value(&mut pair.into_inner())?);
            } else {
                return Err(MohoError::ParseFieldError(
                    format!("Cannot parse {:?} in field.", pair),
                    span,
                ));
            }
        }

        Ok(Declaration::Field(result))
    }

    pub fn parse_type(pair: Pair<'_, Rule>) -> Result<Type, MohoError> {
        let str = pair.as_str();
        let span = pair.as_span();
        match pair.as_rule() {
            Rule::identifier if pair.as_str() == "bool" => Ok(Type::Bool),
            Rule::identifier if pair.as_str() == "char" => Ok(Type::Char),
            Rule::identifier if pair.as_str() == "float" => Ok(Type::Float),
            Rule::identifier if pair.as_str() == "double" => Ok(Type::Float),
            Rule::identifier if pair.as_str() == "int" => Ok(Type::Integer),
            Rule::identifier if pair.as_str() == "long" => Ok(Type::Integer),
            Rule::identifier if pair.as_str() == "string" => Ok(Type::String),
            Rule::identifier => Ok(Type::Class(pair.as_str().to_string())),

            _ => {
                if let Some(typ) = pair.into_inner().next() {
                    match typ.as_rule() {
                        Rule::pointer_type => Ok(Type::Pointer(Box::new(Self::parse_type(
                            typ.into_inner().next().unwrap(),
                        )?))),
                        Rule::reference_type => Ok(Type::Reference(Box::new(Self::parse_type(
                            typ.into_inner().next().unwrap(),
                        )?))),
                        Rule::matrix_type => Ok(Type::Matrix(Box::new(Self::parse_type(
                            typ.into_inner().next().unwrap(),
                        )?))),
                        Rule::array_type => Ok(Type::Array(Box::new(Self::parse_type(
                            typ.into_inner().next().unwrap(),
                        )?))),
                        Rule::identifier if typ.as_str() == "bool" => Ok(Type::Bool),
                        Rule::identifier if typ.as_str() == "char" => Ok(Type::Char),
                        Rule::identifier if typ.as_str() == "float" => Ok(Type::Float),
                        Rule::identifier if typ.as_str() == "double" => Ok(Type::Float),
                        Rule::identifier if typ.as_str() == "int" => Ok(Type::Integer),
                        Rule::identifier if typ.as_str() == "long" => Ok(Type::Integer),
                        Rule::identifier if typ.as_str() == "string" => Ok(Type::String),
                        Rule::identifier => Ok(Type::Class(typ.as_str().to_string())),

                        _ => {
                            return Err(MohoError::ParseTypeError(
                                format!("Unknown type {:?}.", str),
                                span,
                            ));
                        }
                    }
                } else {
                    Err(MohoError::ParseTypeError(
                        format!("Parsing type {:?} failed.", str),
                        span,
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod parsing_tests {
    use super::MohoParser;

    #[test]
    pub fn test_parsing() {
        assert!(MohoParser::apply(include_str!("../assets/single_class.moho")).is_ok());
        assert!(MohoParser::apply(include_str!("../assets/multi_classes.moho")).is_ok());
    }
}
