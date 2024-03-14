use std::{borrow::Cow, ops::Deref};

use serde_json::Value;

use crate::{
    registry::{MetaSchemaRef, Registry},
    types::{ParseFromJSON, ParseFromParameter, ParseResult, ToJSON, Type},
};

impl<'a, T: ToOwned + ?Sized + 'a> Type for Cow<'a, T>
where &'a T : Type, <T as ToOwned>::Owned: Sync + Send + Type {
    const IS_REQUIRED: bool = true;
    type RawValueType = Self;
    type RawElementValueType = Self;

    fn name() -> Cow<'static, str> { <&'a T as Type>::name() }
    fn schema_ref() -> MetaSchemaRef { <&'a T as Type>::schema_ref() }
    fn register(registry: &mut Registry) { <&'a T as Type>::register(registry); }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> { Some(self) }

    fn raw_element_iter<'b>(
        &'b self,
    ) -> Box<dyn Iterator<Item = &'b Self::RawElementValueType> + 'b> {
        Box::new(Some(self).into_iter())
    }
}

impl<'a, T: ToOwned + ?Sized + 'a> ParseFromJSON for Cow<'a, T>
where &'a T : Type, <T as ToOwned>::Owned: Sync + Send + ParseFromJSON {
    fn parse_from_json(value: Option<Value>) -> ParseResult<Self> {
        match T::Owned::parse_from_json(value) {
            Ok(v) => Ok(Cow::Owned(v)),
            Err(e) => Err(e.propagate()),
        }
    }
}

impl<'a, T: ToOwned + ?Sized + 'a> ParseFromParameter for Cow<'a, T>
where &'a T : Type, <T as ToOwned>::Owned: Sync + Send + ParseFromParameter {
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

impl<T: ToOwned + ?Sized> ToJSON for Cow<'_, T>
where for<'a> &'a T : Type + ToJSON, <T as ToOwned>::Owned: Sync + Send + Type {
    fn to_json(&self, v: i32) -> Option<Value> {
        self.deref().to_json(v)
    }
}
