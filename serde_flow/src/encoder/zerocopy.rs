use std::marker::PhantomData;

use crate::error::SerdeFlowError;
use rkyv::{ser::Serializer, Archive, Serialize};

pub type DefaultSerializer = rkyv::ser::serializers::AllocSerializer<4096>;

pub struct Encoder;

impl Encoder {
    pub fn serialize<T: Archive>(value: T) -> Result<Vec<u8>, crate::error::SerdeFlowError>
    where
        T: Serialize<DefaultSerializer>,
    {
        let mut serializer = DefaultSerializer::default();
        let _ = serializer.serialize_value(&value).unwrap();
        let bytes = serializer.into_serializer().into_inner().into_vec();
        Ok(bytes)
    }
}

pub struct Reader<T>
where
    T: rkyv::Archive,
    T::Archived: for<'b> rkyv::CheckBytes<rkyv::validation::validators::DefaultValidator<'b>>,
{
    bytes: Vec<u8>,
    phantom: PhantomData<T>,
}

impl<T> Reader<T>
where
    T: rkyv::Archive,
    T::Archived: for<'b> rkyv::CheckBytes<rkyv::validation::validators::DefaultValidator<'b>>,
{
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            phantom: PhantomData,
        }
    }

    pub fn archive(&self) -> Result<&T::Archived, SerdeFlowError> {
        rkyv::check_archived_root::<T>(&self.bytes).map_err(|_| SerdeFlowError::ParsingFailed)
    }
}
