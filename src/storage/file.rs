use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub enum FileHeaderSpec {
    Null,
    Bool,
    Integer,
    UInteger,
    Float,
    Text,
    Key { name: String, value_type: Box<FileHeaderSpec> },
    Array { allowed_types: Box<Vec<FileHeaderSpec>> },
}

#[derive(Debug)]
pub enum FileHeaderData {
    Null,
    Bool(bool),
    Integer(i64),
    UInteger(u64),
    Float(f64),
    Text(String),
    Key { name: String, value: Box<FileHeaderData> },
    Array { values: Box<Vec<FileHeaderData>> },
}

pub trait FileData : Sized {
    fn get_fulltext(&self) -> String;
    fn get_abbrev(&self) -> String;
}

impl Display for FileHeaderSpec {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            &FileHeaderSpec::Null       => write!(fmt, "NULL"),
            &FileHeaderSpec::Bool       => write!(fmt, "Bool"),
            &FileHeaderSpec::Integer    => write!(fmt, "Integer"),
            &FileHeaderSpec::UInteger   => write!(fmt, "UInteger"),
            &FileHeaderSpec::Float      => write!(fmt, "Float"),
            &FileHeaderSpec::Text       => write!(fmt, "Text"),
            &FileHeaderSpec::Key{name: ref n, value_type: ref vt} => {
                write!(fmt, "Key({:?}) -> {:?}", n, vt)
            }
            &FileHeaderSpec::Array{allowed_types: ref at}  => {
                write!(fmt, "Array({:?})", at)
            }
        }
    }

}

pub struct MatchError {
    summary: String,
    path: Vec<FileHeaderSpec>,
    expected: FileHeaderSpec,
    found: FileHeaderSpec
}

impl MatchError {
    pub fn format(&self) -> String {
        format!("MatchError: {:?}\n\nHaving: {:?}\nExpected: {:?}\nFound: {:?}\n",
               self.summary, self.path, self.expected, self.found)
    }
}

impl Error for MatchError {

    fn description(&self) -> &str {
        &self.summary[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl Debug for MatchError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.format());
        Ok(())
    }

}

impl Display for MatchError {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.format());
        Ok(())
    }

}

pub fn match_header_spec(spec: &FileHeaderSpec, data: &FileHeaderData)
    -> Option<MatchError>
{
    let tpl = (spec, data);
    match tpl {
        (&FileHeaderSpec::Null,     &FileHeaderData::Null)           => { }
        (&FileHeaderSpec::Bool,     &FileHeaderData::Bool(_))        => { }
        (&FileHeaderSpec::Integer,  &FileHeaderData::Integer(_))     => { }
        (&FileHeaderSpec::UInteger, &FileHeaderData::UInteger(_))    => { }
        (&FileHeaderSpec::Float,    &FileHeaderData::Float(_))       => { }
        (&FileHeaderSpec::Text,     &FileHeaderData::Text(_))        => { }

        (
            &FileHeaderSpec::Key{name: ref kname, value_type: ref vtype},
            &FileHeaderData::Key{name: ref n, value: ref val}
        ) => {
            if kname != n {
                // error
            }
            return match_header_spec(&*vtype, &*val);
        }

        (
            &FileHeaderSpec::Array{allowed_types: ref vtypes},
            &FileHeaderData::Array{values: ref vs}
        ) => {
            for (t, v) in vtypes.iter().zip(vs.iter()) {
                let res = match_header_spec(t, v);
                if res.is_some() {
                    return res;
                }
            }
        }

        _ => { unreachable!() }
    }
    None
}

