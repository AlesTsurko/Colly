mod function;
mod mixer;
mod pattern;

pub use function::*;
pub use mixer::*;
pub use pattern::*;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

type PremitiveResult<T> = Result<T, PrimitiveError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Value<T>(pub T);

#[derive(Debug, Clone)]
pub enum ValueWrapper {
    Identifier(Value<Identifier>),
    Boolean(Value<bool>),
    Number(Value<f64>),
    String(Value<String>),
    Properties(Value<Properties>),
    Array(Value<Vec<ValueWrapper>>),
    Function(Value<Box<Function<Item = ValueWrapper>>>),
    Pattern(Value<Pattern>),
    Mixer, // we'll get mixer from context
    Track(Value<Rc<Track>>),
    Slot(Value<Rc<Slot>>),
    Void(Value<()>),
    Nothing,
}

pub enum TypeId {
    Identifier,
    Boolean,
    Number,
    Pattern,
    String,
    Properties,
    Array,
    Function,
    Mixer,
    Track,
    Slot,
    Void,
    Nothing,
}

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TypeId::*;
        match self {
            Identifier => write!(f, "<Identifier>"),
            Boolean => write!(f, "<Boolean>"),
            Number => write!(f, "<Number>"),
            Pattern => write!(f, "<Pattern>"),
            String => write!(f, "<String>"),
            Properties => write!(f, "<Properties>"),
            Array => write!(f, "<Array>"),
            Function => write!(f, "<Function>"),
            Mixer => write!(f, "<Mixer>"),
            Track => write!(f, "<Track>"),
            Slot => write!(f, "<Slot>"),
            Void => write!(f, "<Void>"),
            Nothing => write!(f, "<Nothing>"),
        }
    }
}

macro_rules! impl_from_for_value_wrapper {
    ($from:ty, $item:ident) => {
        impl From<$from> for ValueWrapper {
            fn from(value: $from) -> Self {
                ValueWrapper::$item(Value(value))
            }
        }
    };
}

impl_from_for_value_wrapper!(Identifier, Identifier);
impl_from_for_value_wrapper!(bool, Boolean);
impl_from_for_value_wrapper!(f64, Number);
impl_from_for_value_wrapper!(String, String);
impl_from_for_value_wrapper!(Properties, Properties);
impl_from_for_value_wrapper!(Vec<ValueWrapper>, Array);
impl_from_for_value_wrapper!(Pattern, Pattern);
impl_from_for_value_wrapper!(Rc<Track>, Track);
impl_from_for_value_wrapper!(Rc<Slot>, Slot);
impl_from_for_value_wrapper!(Box<Function<Item = ValueWrapper>>, Function);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct Properties(pub HashMap<Identifier, ValueWrapper>);

pub trait HasProperties {
    fn property(&self, key: &Identifier) -> Option<ValueWrapper>;
    fn set_property(
        &mut self,
        key: &Identifier,
        value: ValueWrapper,
    ) -> PremitiveResult<()>;
}

#[derive(Debug, Fail)]
pub enum PrimitiveError {
    #[fail(
        display = "Cannot set property {} for {}: {}",
        property_name, assignee_name, cause
    )]
    SetProperty {
        property_name: String,
        assignee_name: String,
        cause: String,
    },
}
