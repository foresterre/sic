//! Parsing for named values, which have the following syntax N(T)
//! where N is the identifier of the named value, and T is a comma separated tuple of values,
//! like so: `a,b,c`. Dangling commas are not supported. A full example look like this: `rgb(4, 255, 0)`.

use super::Rule;
use pest::iterators::Pair;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NamedValueError {
    #[error("Named value expected an identifier but none was found")]
    IdentifierNotFound,

    #[error("Found an unknown identifier for named value '{0}'")]
    IdentifierInvalid(String),

    #[error("Named value argument has an incorrect type")]
    InvalidArgumentType,

    #[error("Unable to create named value: no matching arguments for identifier '{0}' found")]
    UnableToCreateNamedValueWithArgs(Ident),

    #[error(
        "Unable to extract arguments: expected named value with identifier '{0}' but found '{1}'"
    )]
    UnableToExtractNamedValueArgs(String, String),

    #[error("Unable to extract value: expected value of type '{0}' but found '{1}'")]
    UnableToExtractValue(String, String),

    #[error("Unable to parse value '{0}', with type '{1}'")]
    UnableToParse(String, String),

    #[error("This error has to be defined, TODO <3")]
    TODOError,
}

type NVResult<T> = Result<T, NamedValueError>;

pub fn parse_named_value(pair: Pair<'_, Rule>) -> NVResult<NamedValue> {
    let mut pairs = pair.into_inner();

    // identifier
    let ident = pairs
        .next()
        .ok_or_else(|| NamedValueError::IdentifierNotFound)?;

    let ident = match ident.as_rule() {
        Rule::ident => parse_ident(ident)?,
        _ => return Err(NamedValueError::IdentifierNotFound),
    };

    // arguments
    let arguments = pairs
        .map(|pair| Value::try_from_pair(pair, ident))
        .collect::<NVResult<Vec<_>>>()?;

    NamedValue::try_from_annotated(AnnotatedArgs { ident, arguments })
}

#[derive(Debug, Clone, Copy)]
pub enum Ident {
    // rgba(<u8>,<u8>,<u8>,<u8>)
    Rgba,

    // size(<u32>)
    Size,

    // font("<path>")
    Font,
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rgba => f.write_str("Rgba"),
            Self::Size => f.write_str("Size"),
            Self::Font => f.write_str("Font"),
        }
    }
}

fn parse_ident(pair: Pair<'_, Rule>) -> NVResult<Ident> {
    let ident = match pair.as_str() {
        "rgba" => Ident::Rgba,
        "size" => Ident::Size,
        "font" => Ident::Font,
        _ => {
            return Err(NamedValueError::IdentifierInvalid(
                pair.as_str().to_string(),
            ))
        }
    };

    Ok(ident)
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Byte(u8),
    Float(f32),
    Integer(i32),
    NatNum(u32),
    String(&'a str),
}

impl<'a> Value<'a> {
    pub fn try_from_pair(pair: Pair<'a, Rule>, ident: Ident) -> NVResult<Self> {
        match (pair.as_rule(), ident) {
            (Rule::fp, Ident::Rgba) => Ok(Value::parse_byte(pair.as_str())?),
            (Rule::fp, Ident::Size) => Ok(Value::parse_float(pair.as_str())?),
            (Rule::string_unicode, _) => Ok(Value::parse_string(pair.into_inner().as_str())?),
            _ => Err(NamedValueError::InvalidArgumentType),
        }
    }

    pub fn extract_byte(&self) -> NVResult<u8> {
        if let Self::Byte(inner) = self {
            Ok(*inner)
        } else {
            Err(NamedValueError::UnableToExtractValue(
                String::from("Byte"),
                self.error_type(),
            ))
        }
    }

    pub fn extract_float(&self) -> NVResult<f32> {
        if let Self::Float(inner) = self {
            Ok(*inner)
        } else {
            Err(NamedValueError::UnableToExtractValue(
                String::from("Float"),
                self.error_type(),
            ))
        }
    }

    pub fn extract_integer(&self) -> NVResult<i32> {
        if let Self::Integer(inner) = self {
            Ok(*inner)
        } else {
            Err(NamedValueError::UnableToExtractValue(
                String::from("Integer"),
                self.error_type(),
            ))
        }
    }

    pub fn extract_nat_num(&self) -> NVResult<u32> {
        if let Self::NatNum(inner) = self {
            Ok(*inner)
        } else {
            Err(NamedValueError::UnableToExtractValue(
                String::from("NatNum"),
                self.error_type(),
            ))
        }
    }

    pub fn extract_string(&self) -> NVResult<&str> {
        if let Self::String(inner) = self {
            Ok(inner)
        } else {
            Err(NamedValueError::UnableToExtractValue(
                String::from("String"),
                self.error_type(),
            ))
        }
    }

    fn parse_byte(value: &str) -> NVResult<Self> {
        value
            .parse::<u8>()
            .map(Value::Byte)
            .map_err(|_err| NamedValueError::UnableToParse(value.to_string(), String::from("Byte")))
    }

    fn parse_float(value: &str) -> NVResult<Self> {
        value
            .parse::<f32>()
            .map(Value::Float)
            .map_err(|_err| NamedValueError::UnableToParse(value.to_string(), String::from("Byte")))
    }

    fn parse_string(value: &'a str) -> NVResult<Self> {
        Ok(Value::String(value))
    }

    fn error_type(&self) -> String {
        let typ = match self {
            Self::Byte(_) => "Byte",
            Self::Float(_) => "Float",
            Self::Integer(_) => "Integer",
            Self::NatNum(_) => "NatNum",
            Self::String(_) => "String",
        };

        typ.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct AnnotatedArgs<'a> {
    ident: Ident,
    arguments: Vec<Value<'a>>,
}

impl<'a> AnnotatedArgs<'a> {
    fn ident(&self) -> Ident {
        self.ident
    }

    // moves arguments!
    fn arguments(&self) -> &[Value] {
        self.arguments.as_slice()
    }
}

#[derive(Debug, Clone)]
pub enum NamedValue {
    Rgba(u8, u8, u8, u8),
    Size(f32),
    Font(PathBuf),
}

impl NamedValue {
    pub fn try_from_annotated(args: AnnotatedArgs) -> NVResult<Self> {
        match args.ident() {
            Ident::Rgba => NamedValue::create_rgba(args.arguments()),
            Ident::Size => NamedValue::create_size(args.arguments()),
            Ident::Font => NamedValue::create_font(args.arguments()),
        }
    }

    pub fn extract_rgba(&self) -> NVResult<[u8; 4]> {
        if let Self::Rgba(r, g, b, a) = self {
            Ok([*r, *g, *b, *a])
        } else {
            Err(NamedValueError::UnableToExtractValue(
                String::from("Rgba"),
                self.error_type(),
            ))
        }
    }

    pub fn extract_size(&self) -> NVResult<f32> {
        if let Self::Size(size) = self {
            Ok(*size)
        } else {
            Err(NamedValueError::UnableToExtractValue(
                String::from("Size"),
                self.error_type(),
            ))
        }
    }

    pub fn extract_font(&self) -> NVResult<PathBuf> {
        if let Self::Font(font) = self {
            Ok(font.to_path_buf())
        } else {
            Err(NamedValueError::UnableToExtractValue(
                String::from("Font"),
                self.error_type(),
            ))
        }
    }

    fn create_rgba(args: &[Value]) -> NVResult<Self> {
        match args {
            [r, g, b, a] => Ok(Self::Rgba(
                r.extract_byte()?,
                g.extract_byte()?,
                b.extract_byte()?,
                a.extract_byte()?,
            )),
            _ => Err(NamedValueError::UnableToCreateNamedValueWithArgs(
                Ident::Rgba,
            )),
        }
    }

    fn create_size(args: &[Value]) -> NVResult<Self> {
        match args {
            [size] => Ok(Self::Size(size.extract_float()?)),
            _ => Err(NamedValueError::UnableToCreateNamedValueWithArgs(
                Ident::Size,
            )),
        }
    }

    fn create_font(args: &[Value]) -> NVResult<Self> {
        match args {
            [font_file] => Ok(Self::Font(font_file.extract_string()?.into())),
            _ => Err(NamedValueError::UnableToCreateNamedValueWithArgs(
                Ident::Font,
            )),
        }
    }

    fn error_type(&self) -> String {
        let typ = match self {
            Self::Rgba(_, _, _, _) => "Rgba",
            Self::Size(_) => "Size",
            Self::Font(_) => "Font",
        };

        typ.to_string()
    }
}
