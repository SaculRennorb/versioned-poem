use std::{borrow::Cow, ops::Deref};

use serde_json::Value;

use crate::{
    registry::{MetaSchemaRef, Registry},
    types::{ParseFromJSON, ParseFromParameter, ParseResult, ToJSON, Type},
};

impl<T: ToOwned + Type> Type for Cow<'_, T> where <T as ToOwned>::Owned: Sync + Send {
    const IS_REQUIRED: bool = true;
    type RawValueType = T::RawValueType;
    type RawElementValueType = T::RawElementValueType;

    fn name() -> Cow<'static, str> { T::name() }
    fn schema_ref() -> MetaSchemaRef { T::schema_ref() }
    fn register(registry: &mut Registry) { T::register(registry); }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> { self.deref().as_raw_value() }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        self.deref().raw_element_iter()
    }
}

impl<T: ToOwned + ParseFromJSON> ParseFromJSON for Cow<'_, T>  where <T as ToOwned>::Owned: Sync + Send + ParseFromJSON {
    fn parse_from_json(value: Option<Value>) -> ParseResult<Self> {
        match T::Owned::parse_from_json(value) {
            Ok(v) => Ok(Cow::Owned(v)),
            Err(e) => Err(e.propagate()),
        }
    }
}

impl<T: ToOwned + ParseFromParameter> ParseFromParameter for Cow<'_, T>  where <T as ToOwned>::Owned: Sync + Send + ParseFromParameter {
    fn parse_from_parameter(_value: &str) -> ParseResult<Self> {
        match T::Owned::parse_from_parameter(_value) {
            Ok(v) => Ok(Cow::Owned(v)),
            Err(e) => Err(e.propagate()),
        }
    }

    fn parse_from_parameters<I: IntoIterator<Item = A>, A: AsRef<str>>(
        iter: I,
    ) -> ParseResult<Self> {
        match T::Owned::parse_from_parameters(iter) {
            Ok(v) => Ok(Cow::Owned(v)),
            Err(e) => Err(e.propagate()),
        }
    }
}

impl<T: ToOwned + ToJSON> ToJSON for Cow<'_, T>  where <T as ToOwned>::Owned: Sync + Send {
    fn to_json(&self, v: i32) -> Option<Value> {
        self.deref().to_json(v)
    }
}
