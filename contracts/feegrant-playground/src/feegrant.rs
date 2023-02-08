use cosmwasm_schema::{
    cw_serde,
    serde::{Deserialize, Serialize},
};
use prost::EncodeError;
use schemars::JsonSchema;


#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
#[serde(crate = "::cosmwasm_schema::serde")]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
#[serde(crate = "::cosmwasm_schema::serde")]
pub struct Any {
    #[prost(string, tag = "1")]
    pub type_url: String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: Vec<u8>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
#[serde(crate = "::cosmwasm_schema::serde")]
pub struct MsgGrantAllowance {
    #[prost(string, tag = "1")]
    pub granter: String,
    #[prost(string, tag = "2")]
    pub grantee: String,
    #[prost(message, optional, tag = "3")]
    pub allowance: Option<Any>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
#[serde(crate = "::cosmwasm_schema::serde")]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
#[serde(crate = "::cosmwasm_schema::serde")]
pub struct BasicAllowance {
    #[prost(message, repeated, tag = "1")]
    pub spend_limit: Vec<Coin>,
    #[prost(message, optional, tag = "2")]
    pub expiration: Option<Timestamp>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
#[serde(crate = "::cosmwasm_schema::serde")]
pub struct PeriodicAllowance {
    #[prost(message, optional, tag = "1")]
    pub basic: Option<BasicAllowance>,
    #[prost(message, optional, tag = "2")]
    pub period: Option<Duration>,
    #[prost(message, repeated, tag = "3")]
    pub period_spend_limit: Vec<Coin>,
    #[prost(message, repeated, tag = "4")]
    pub period_can_spend: Vec<Coin>,
    #[prost(message, optional, tag = "5")]
    pub period_reset: Option<Timestamp>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
#[serde(crate = "::cosmwasm_schema::serde")]
pub struct Duration {
    /// Signed seconds of the span of time. Must be from -315,576,000,000
    /// to +315,576,000,000 inclusive. Note: these bounds are computed from:
    /// 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Signed fractions of a second at nanosecond resolution of the span
    /// of time. Durations less than one second are represented with a 0
    /// `seconds` field and a positive or negative `nanos` field. For durations
    /// of one second or more, a non-zero value for the `nanos` field must be
    /// of the same sign as the `seconds` field. Must be from -999,999,999
    /// to +999,999,999 inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

#[cw_serde]
pub enum Allowance {
    BasicAllowance(BasicAllowance),
    PeriodicAllowance(PeriodicAllowance),
}

pub trait MessageExt: prost::Message {
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError>;
}
impl<M> MessageExt for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
        let mut bytes = Vec::new();
        prost::Message::encode(self, &mut bytes)?;
        Ok(bytes)
    }
}