// @generated
impl serde::Serialize for AggProofRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start != 0 {
            len += 1;
        }
        if self.end != 0 {
            len += 1;
        }
        if self.l1_block_number != 0 {
            len += 1;
        }
        if !self.l1_block_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("proofs.AggProofRequest", len)?;
        if self.start != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("start", ToString::to_string(&self.start).as_str())?;
        }
        if self.end != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("end", ToString::to_string(&self.end).as_str())?;
        }
        if self.l1_block_number != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("l1BlockNumber", ToString::to_string(&self.l1_block_number).as_str())?;
        }
        if !self.l1_block_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("l1BlockHash", pbjson::private::base64::encode(&self.l1_block_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggProofRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
            "l1_block_number",
            "l1BlockNumber",
            "l1_block_hash",
            "l1BlockHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
            L1BlockNumber,
            L1BlockHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            "l1BlockNumber" | "l1_block_number" => Ok(GeneratedField::L1BlockNumber),
                            "l1BlockHash" | "l1_block_hash" => Ok(GeneratedField::L1BlockHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AggProofRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct proofs.AggProofRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AggProofRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                let mut l1_block_number__ = None;
                let mut l1_block_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::L1BlockNumber => {
                            if l1_block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("l1BlockNumber"));
                            }
                            l1_block_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::L1BlockHash => {
                            if l1_block_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("l1BlockHash"));
                            }
                            l1_block_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AggProofRequest {
                    start: start__.unwrap_or_default(),
                    end: end__.unwrap_or_default(),
                    l1_block_number: l1_block_number__.unwrap_or_default(),
                    l1_block_hash: l1_block_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("proofs.AggProofRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AggProofResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success {
            len += 1;
        }
        if !self.error.is_empty() {
            len += 1;
        }
        if self.request_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("proofs.AggProofResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if !self.error.is_empty() {
            struct_ser.serialize_field("error", &self.error)?;
        }
        if self.request_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("requestId", ToString::to_string(&self.request_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggProofResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "error",
            "request_id",
            "requestId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            Error,
            RequestId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "success" => Ok(GeneratedField::Success),
                            "error" => Ok(GeneratedField::Error),
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AggProofResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct proofs.AggProofResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AggProofResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                let mut error__ = None;
                let mut request_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AggProofResponse {
                    success: success__.unwrap_or_default(),
                    error: error__.unwrap_or_default(),
                    request_id: request_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("proofs.AggProofResponse", FIELDS, GeneratedVisitor)
    }
}
