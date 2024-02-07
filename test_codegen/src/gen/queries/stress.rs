// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#[derive(Debug)]
pub struct EverythingParams<
    T1: cornucopia_async::StringSql,
    T2: cornucopia_async::StringSql,
    T3: cornucopia_async::BytesSql,
    T4: cornucopia_async::JsonSql,
    T5: cornucopia_async::JsonSql,
> {
    pub bool_: bool,
    pub boolean_: bool,
    pub char_: i8,
    pub smallint_: i16,
    pub int2_: i16,
    pub smallserial_: i16,
    pub serial2_: i16,
    pub int_: i32,
    pub int4_: i32,
    pub serial_: i32,
    pub serial4_: i32,
    pub bingint_: i64,
    pub int8_: i64,
    pub bigserial_: i64,
    pub serial8_: i64,
    pub float4_: f32,
    pub real_: f32,
    pub float8_: f64,
    pub double_precision_: f64,
    pub text_: T1,
    pub varchar_: T2,
    pub bytea_: T3,
    pub timestamp_: chrono::NaiveDateTime,
    pub timestamp_without_time_zone_: chrono::NaiveDateTime,
    pub timestamptz_: chrono::DateTime<chrono::Utc>,
    pub timestamp_with_time_zone_: chrono::DateTime<chrono::Utc>,
    pub date_: chrono::NaiveDate,
    pub time_: chrono::NaiveTime,
    pub json_: T4,
    pub jsonb_: T5,
    pub uuid_: uuid::Uuid,
    pub inet_: std::net::IpAddr,
    pub macaddr_: eui48::MacAddress,
    pub numeric_: rust_decimal::Decimal,
}
#[derive(Debug)]
pub struct EverythingArrayParams<
    T1: cornucopia_async::ArraySql<Item = bool>,
    T2: cornucopia_async::ArraySql<Item = bool>,
    T3: cornucopia_async::ArraySql<Item = i8>,
    T4: cornucopia_async::ArraySql<Item = i16>,
    T5: cornucopia_async::ArraySql<Item = i16>,
    T6: cornucopia_async::ArraySql<Item = i32>,
    T7: cornucopia_async::ArraySql<Item = i32>,
    T8: cornucopia_async::ArraySql<Item = i64>,
    T9: cornucopia_async::ArraySql<Item = i64>,
    T10: cornucopia_async::ArraySql<Item = f32>,
    T11: cornucopia_async::ArraySql<Item = f32>,
    T12: cornucopia_async::ArraySql<Item = f64>,
    T13: cornucopia_async::ArraySql<Item = f64>,
    T14: cornucopia_async::StringSql,
    T15: cornucopia_async::ArraySql<Item = T14>,
    T16: cornucopia_async::StringSql,
    T17: cornucopia_async::ArraySql<Item = T16>,
    T18: cornucopia_async::BytesSql,
    T19: cornucopia_async::ArraySql<Item = T18>,
    T20: cornucopia_async::ArraySql<Item = chrono::NaiveDateTime>,
    T21: cornucopia_async::ArraySql<Item = chrono::NaiveDateTime>,
    T22: cornucopia_async::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
    T23: cornucopia_async::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
    T24: cornucopia_async::ArraySql<Item = chrono::NaiveDate>,
    T25: cornucopia_async::ArraySql<Item = chrono::NaiveTime>,
    T26: cornucopia_async::JsonSql,
    T27: cornucopia_async::ArraySql<Item = T26>,
    T28: cornucopia_async::JsonSql,
    T29: cornucopia_async::ArraySql<Item = T28>,
    T30: cornucopia_async::ArraySql<Item = uuid::Uuid>,
    T31: cornucopia_async::ArraySql<Item = std::net::IpAddr>,
    T32: cornucopia_async::ArraySql<Item = eui48::MacAddress>,
    T33: cornucopia_async::ArraySql<Item = rust_decimal::Decimal>,
> {
    pub bool_: T1,
    pub boolean_: T2,
    pub char_: T3,
    pub smallint_: T4,
    pub int2_: T5,
    pub int_: T6,
    pub int4_: T7,
    pub bingint_: T8,
    pub int8_: T9,
    pub float4_: T10,
    pub real_: T11,
    pub float8_: T12,
    pub double_precision_: T13,
    pub text_: T15,
    pub varchar_: T17,
    pub bytea_: T19,
    pub timestamp_: T20,
    pub timestamp_without_time_zone_: T21,
    pub timestamptz_: T22,
    pub timestamp_with_time_zone_: T23,
    pub date_: T24,
    pub time_: T25,
    pub json_: T27,
    pub jsonb_: T29,
    pub uuid_: T30,
    pub inet_: T31,
    pub macaddr_: T32,
    pub numeric_: T33,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct Everything {
    pub bool_: bool,
    pub boolean_: bool,
    pub char_: i8,
    pub smallint_: i16,
    pub int2_: i16,
    pub smallserial_: i16,
    pub serial2_: i16,
    pub int_: i32,
    pub int4_: i32,
    pub serial_: i32,
    pub serial4_: i32,
    pub bingint_: i64,
    pub int8_: i64,
    pub bigserial_: i64,
    pub serial8_: i64,
    pub float4_: f32,
    pub real_: f32,
    pub float8_: f64,
    pub double_precision_: f64,
    pub text_: String,
    pub varchar_: String,
    pub bytea_: Vec<u8>,
    pub timestamp_: chrono::NaiveDateTime,
    pub timestamp_without_time_zone_: chrono::NaiveDateTime,
    pub timestamptz_: chrono::DateTime<chrono::Utc>,
    pub timestamp_with_time_zone_: chrono::DateTime<chrono::Utc>,
    pub date_: chrono::NaiveDate,
    pub time_: chrono::NaiveTime,
    pub json_: serde_json::Value,
    pub jsonb_: serde_json::Value,
    pub uuid_: uuid::Uuid,
    pub inet_: std::net::IpAddr,
    pub macaddr_: eui48::MacAddress,
    pub numeric_: rust_decimal::Decimal,
}
pub struct EverythingBorrowed<'a> {
    pub bool_: bool,
    pub boolean_: bool,
    pub char_: i8,
    pub smallint_: i16,
    pub int2_: i16,
    pub smallserial_: i16,
    pub serial2_: i16,
    pub int_: i32,
    pub int4_: i32,
    pub serial_: i32,
    pub serial4_: i32,
    pub bingint_: i64,
    pub int8_: i64,
    pub bigserial_: i64,
    pub serial8_: i64,
    pub float4_: f32,
    pub real_: f32,
    pub float8_: f64,
    pub double_precision_: f64,
    pub text_: &'a str,
    pub varchar_: &'a str,
    pub bytea_: &'a [u8],
    pub timestamp_: chrono::NaiveDateTime,
    pub timestamp_without_time_zone_: chrono::NaiveDateTime,
    pub timestamptz_: chrono::DateTime<chrono::Utc>,
    pub timestamp_with_time_zone_: chrono::DateTime<chrono::Utc>,
    pub date_: chrono::NaiveDate,
    pub time_: chrono::NaiveTime,
    pub json_: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub jsonb_: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub uuid_: uuid::Uuid,
    pub inet_: std::net::IpAddr,
    pub macaddr_: eui48::MacAddress,
    pub numeric_: rust_decimal::Decimal,
}
impl<'a> From<EverythingBorrowed<'a>> for Everything {
    fn from(
        EverythingBorrowed {
            bool_,
            boolean_,
            char_,
            smallint_,
            int2_,
            smallserial_,
            serial2_,
            int_,
            int4_,
            serial_,
            serial4_,
            bingint_,
            int8_,
            bigserial_,
            serial8_,
            float4_,
            real_,
            float8_,
            double_precision_,
            text_,
            varchar_,
            bytea_,
            timestamp_,
            timestamp_without_time_zone_,
            timestamptz_,
            timestamp_with_time_zone_,
            date_,
            time_,
            json_,
            jsonb_,
            uuid_,
            inet_,
            macaddr_,
            numeric_,
        }: EverythingBorrowed<'a>,
    ) -> Self {
        Self {
            bool_,
            boolean_,
            char_,
            smallint_,
            int2_,
            smallserial_,
            serial2_,
            int_,
            int4_,
            serial_,
            serial4_,
            bingint_,
            int8_,
            bigserial_,
            serial8_,
            float4_,
            real_,
            float8_,
            double_precision_,
            text_: text_.into(),
            varchar_: varchar_.into(),
            bytea_: bytea_.into(),
            timestamp_,
            timestamp_without_time_zone_,
            timestamptz_,
            timestamp_with_time_zone_,
            date_,
            time_,
            json_: serde_json::from_str(json_.0.get()).unwrap(),
            jsonb_: serde_json::from_str(jsonb_.0.get()).unwrap(),
            uuid_,
            inet_,
            macaddr_,
            numeric_,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct EverythingNull {
    pub bool_: Option<bool>,
    pub boolean_: Option<bool>,
    pub char_: Option<i8>,
    pub smallint_: Option<i16>,
    pub int2_: Option<i16>,
    pub smallserial_: Option<i16>,
    pub serial2_: Option<i16>,
    pub int_: Option<i32>,
    pub int4_: Option<i32>,
    pub serial_: Option<i32>,
    pub serial4_: Option<i32>,
    pub bingint_: Option<i64>,
    pub int8_: Option<i64>,
    pub bigserial_: Option<i64>,
    pub serial8_: Option<i64>,
    pub float4_: Option<f32>,
    pub real_: Option<f32>,
    pub float8_: Option<f64>,
    pub double_precision_: Option<f64>,
    pub text_: Option<String>,
    pub varchar_: Option<String>,
    pub bytea_: Option<Vec<u8>>,
    pub timestamp_: Option<chrono::NaiveDateTime>,
    pub timestamp_without_time_zone_: Option<chrono::NaiveDateTime>,
    pub timestamptz_: Option<chrono::DateTime<chrono::Utc>>,
    pub timestamp_with_time_zone_: Option<chrono::DateTime<chrono::Utc>>,
    pub date_: Option<chrono::NaiveDate>,
    pub time_: Option<chrono::NaiveTime>,
    pub json_: Option<serde_json::Value>,
    pub jsonb_: Option<serde_json::Value>,
    pub uuid_: Option<uuid::Uuid>,
    pub inet_: Option<std::net::IpAddr>,
    pub macaddr_: Option<eui48::MacAddress>,
    pub numeric_: Option<rust_decimal::Decimal>,
}
pub struct EverythingNullBorrowed<'a> {
    pub bool_: Option<bool>,
    pub boolean_: Option<bool>,
    pub char_: Option<i8>,
    pub smallint_: Option<i16>,
    pub int2_: Option<i16>,
    pub smallserial_: Option<i16>,
    pub serial2_: Option<i16>,
    pub int_: Option<i32>,
    pub int4_: Option<i32>,
    pub serial_: Option<i32>,
    pub serial4_: Option<i32>,
    pub bingint_: Option<i64>,
    pub int8_: Option<i64>,
    pub bigserial_: Option<i64>,
    pub serial8_: Option<i64>,
    pub float4_: Option<f32>,
    pub real_: Option<f32>,
    pub float8_: Option<f64>,
    pub double_precision_: Option<f64>,
    pub text_: Option<&'a str>,
    pub varchar_: Option<&'a str>,
    pub bytea_: Option<&'a [u8]>,
    pub timestamp_: Option<chrono::NaiveDateTime>,
    pub timestamp_without_time_zone_: Option<chrono::NaiveDateTime>,
    pub timestamptz_: Option<chrono::DateTime<chrono::Utc>>,
    pub timestamp_with_time_zone_: Option<chrono::DateTime<chrono::Utc>>,
    pub date_: Option<chrono::NaiveDate>,
    pub time_: Option<chrono::NaiveTime>,
    pub json_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub jsonb_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub uuid_: Option<uuid::Uuid>,
    pub inet_: Option<std::net::IpAddr>,
    pub macaddr_: Option<eui48::MacAddress>,
    pub numeric_: Option<rust_decimal::Decimal>,
}
impl<'a> From<EverythingNullBorrowed<'a>> for EverythingNull {
    fn from(
        EverythingNullBorrowed {
            bool_,
            boolean_,
            char_,
            smallint_,
            int2_,
            smallserial_,
            serial2_,
            int_,
            int4_,
            serial_,
            serial4_,
            bingint_,
            int8_,
            bigserial_,
            serial8_,
            float4_,
            real_,
            float8_,
            double_precision_,
            text_,
            varchar_,
            bytea_,
            timestamp_,
            timestamp_without_time_zone_,
            timestamptz_,
            timestamp_with_time_zone_,
            date_,
            time_,
            json_,
            jsonb_,
            uuid_,
            inet_,
            macaddr_,
            numeric_,
        }: EverythingNullBorrowed<'a>,
    ) -> Self {
        Self {
            bool_,
            boolean_,
            char_,
            smallint_,
            int2_,
            smallserial_,
            serial2_,
            int_,
            int4_,
            serial_,
            serial4_,
            bingint_,
            int8_,
            bigserial_,
            serial8_,
            float4_,
            real_,
            float8_,
            double_precision_,
            text_: text_.map(|v| v.into()),
            varchar_: varchar_.map(|v| v.into()),
            bytea_: bytea_.map(|v| v.into()),
            timestamp_,
            timestamp_without_time_zone_,
            timestamptz_,
            timestamp_with_time_zone_,
            date_,
            time_,
            json_: json_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
            jsonb_: jsonb_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
            uuid_,
            inet_,
            macaddr_,
            numeric_,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct EverythingArray {
    pub bool_: Vec<bool>,
    pub boolean_: Vec<bool>,
    pub char_: Vec<i8>,
    pub smallint_: Vec<i16>,
    pub int2_: Vec<i16>,
    pub int_: Vec<i32>,
    pub int4_: Vec<i32>,
    pub bingint_: Vec<i64>,
    pub int8_: Vec<i64>,
    pub float4_: Vec<f32>,
    pub real_: Vec<f32>,
    pub float8_: Vec<f64>,
    pub double_precision_: Vec<f64>,
    pub text_: Vec<String>,
    pub varchar_: Vec<String>,
    pub bytea_: Vec<Vec<u8>>,
    pub timestamp_: Vec<chrono::NaiveDateTime>,
    pub timestamp_without_time_zone_: Vec<chrono::NaiveDateTime>,
    pub timestamptz_: Vec<chrono::DateTime<chrono::Utc>>,
    pub timestamp_with_time_zone_: Vec<chrono::DateTime<chrono::Utc>>,
    pub date_: Vec<chrono::NaiveDate>,
    pub time_: Vec<chrono::NaiveTime>,
    pub json_: Vec<serde_json::Value>,
    pub jsonb_: Vec<serde_json::Value>,
    pub uuid_: Vec<uuid::Uuid>,
    pub inet_: Vec<std::net::IpAddr>,
    pub macaddr_: Vec<eui48::MacAddress>,
    pub numeric_: Vec<rust_decimal::Decimal>,
}
pub struct EverythingArrayBorrowed<'a> {
    pub bool_: cornucopia_async::ArrayIterator<'a, bool>,
    pub boolean_: cornucopia_async::ArrayIterator<'a, bool>,
    pub char_: cornucopia_async::ArrayIterator<'a, i8>,
    pub smallint_: cornucopia_async::ArrayIterator<'a, i16>,
    pub int2_: cornucopia_async::ArrayIterator<'a, i16>,
    pub int_: cornucopia_async::ArrayIterator<'a, i32>,
    pub int4_: cornucopia_async::ArrayIterator<'a, i32>,
    pub bingint_: cornucopia_async::ArrayIterator<'a, i64>,
    pub int8_: cornucopia_async::ArrayIterator<'a, i64>,
    pub float4_: cornucopia_async::ArrayIterator<'a, f32>,
    pub real_: cornucopia_async::ArrayIterator<'a, f32>,
    pub float8_: cornucopia_async::ArrayIterator<'a, f64>,
    pub double_precision_: cornucopia_async::ArrayIterator<'a, f64>,
    pub text_: cornucopia_async::ArrayIterator<'a, &'a str>,
    pub varchar_: cornucopia_async::ArrayIterator<'a, &'a str>,
    pub bytea_: cornucopia_async::ArrayIterator<'a, &'a [u8]>,
    pub timestamp_: cornucopia_async::ArrayIterator<'a, chrono::NaiveDateTime>,
    pub timestamp_without_time_zone_: cornucopia_async::ArrayIterator<'a, chrono::NaiveDateTime>,
    pub timestamptz_: cornucopia_async::ArrayIterator<'a, chrono::DateTime<chrono::Utc>>,
    pub timestamp_with_time_zone_:
        cornucopia_async::ArrayIterator<'a, chrono::DateTime<chrono::Utc>>,
    pub date_: cornucopia_async::ArrayIterator<'a, chrono::NaiveDate>,
    pub time_: cornucopia_async::ArrayIterator<'a, chrono::NaiveTime>,
    pub json_:
        cornucopia_async::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub jsonb_:
        cornucopia_async::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub uuid_: cornucopia_async::ArrayIterator<'a, uuid::Uuid>,
    pub inet_: cornucopia_async::ArrayIterator<'a, std::net::IpAddr>,
    pub macaddr_: cornucopia_async::ArrayIterator<'a, eui48::MacAddress>,
    pub numeric_: cornucopia_async::ArrayIterator<'a, rust_decimal::Decimal>,
}
impl<'a> From<EverythingArrayBorrowed<'a>> for EverythingArray {
    fn from(
        EverythingArrayBorrowed {
            bool_,
            boolean_,
            char_,
            smallint_,
            int2_,
            int_,
            int4_,
            bingint_,
            int8_,
            float4_,
            real_,
            float8_,
            double_precision_,
            text_,
            varchar_,
            bytea_,
            timestamp_,
            timestamp_without_time_zone_,
            timestamptz_,
            timestamp_with_time_zone_,
            date_,
            time_,
            json_,
            jsonb_,
            uuid_,
            inet_,
            macaddr_,
            numeric_,
        }: EverythingArrayBorrowed<'a>,
    ) -> Self {
        Self {
            bool_: bool_.map(|v| v).collect(),
            boolean_: boolean_.map(|v| v).collect(),
            char_: char_.map(|v| v).collect(),
            smallint_: smallint_.map(|v| v).collect(),
            int2_: int2_.map(|v| v).collect(),
            int_: int_.map(|v| v).collect(),
            int4_: int4_.map(|v| v).collect(),
            bingint_: bingint_.map(|v| v).collect(),
            int8_: int8_.map(|v| v).collect(),
            float4_: float4_.map(|v| v).collect(),
            real_: real_.map(|v| v).collect(),
            float8_: float8_.map(|v| v).collect(),
            double_precision_: double_precision_.map(|v| v).collect(),
            text_: text_.map(|v| v.into()).collect(),
            varchar_: varchar_.map(|v| v.into()).collect(),
            bytea_: bytea_.map(|v| v.into()).collect(),
            timestamp_: timestamp_.map(|v| v).collect(),
            timestamp_without_time_zone_: timestamp_without_time_zone_.map(|v| v).collect(),
            timestamptz_: timestamptz_.map(|v| v).collect(),
            timestamp_with_time_zone_: timestamp_with_time_zone_.map(|v| v).collect(),
            date_: date_.map(|v| v).collect(),
            time_: time_.map(|v| v).collect(),
            json_: json_
                .map(|v| serde_json::from_str(v.0.get()).unwrap())
                .collect(),
            jsonb_: jsonb_
                .map(|v| serde_json::from_str(v.0.get()).unwrap())
                .collect(),
            uuid_: uuid_.map(|v| v).collect(),
            inet_: inet_.map(|v| v).collect(),
            macaddr_: macaddr_.map(|v| v).collect(),
            numeric_: numeric_.map(|v| v).collect(),
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct EverythingArrayNull {
    pub bool_: Option<Vec<bool>>,
    pub boolean_: Option<Vec<bool>>,
    pub char_: Option<Vec<i8>>,
    pub smallint_: Option<Vec<i16>>,
    pub int2_: Option<Vec<i16>>,
    pub int_: Option<Vec<i32>>,
    pub int4_: Option<Vec<i32>>,
    pub bingint_: Option<Vec<i64>>,
    pub int8_: Option<Vec<i64>>,
    pub float4_: Option<Vec<f32>>,
    pub real_: Option<Vec<f32>>,
    pub float8_: Option<Vec<f64>>,
    pub double_precision_: Option<Vec<f64>>,
    pub text_: Option<Vec<String>>,
    pub varchar_: Option<Vec<String>>,
    pub bytea_: Option<Vec<Vec<u8>>>,
    pub timestamp_: Option<Vec<chrono::NaiveDateTime>>,
    pub timestamp_without_time_zone_: Option<Vec<chrono::NaiveDateTime>>,
    pub timestamptz_: Option<Vec<chrono::DateTime<chrono::Utc>>>,
    pub timestamp_with_time_zone_: Option<Vec<chrono::DateTime<chrono::Utc>>>,
    pub date_: Option<Vec<chrono::NaiveDate>>,
    pub time_: Option<Vec<chrono::NaiveTime>>,
    pub json_: Option<Vec<serde_json::Value>>,
    pub jsonb_: Option<Vec<serde_json::Value>>,
    pub uuid_: Option<Vec<uuid::Uuid>>,
    pub inet_: Option<Vec<std::net::IpAddr>>,
    pub macaddr_: Option<Vec<eui48::MacAddress>>,
    pub numeric_: Option<Vec<rust_decimal::Decimal>>,
}
pub struct EverythingArrayNullBorrowed<'a> {
    pub bool_: Option<cornucopia_async::ArrayIterator<'a, bool>>,
    pub boolean_: Option<cornucopia_async::ArrayIterator<'a, bool>>,
    pub char_: Option<cornucopia_async::ArrayIterator<'a, i8>>,
    pub smallint_: Option<cornucopia_async::ArrayIterator<'a, i16>>,
    pub int2_: Option<cornucopia_async::ArrayIterator<'a, i16>>,
    pub int_: Option<cornucopia_async::ArrayIterator<'a, i32>>,
    pub int4_: Option<cornucopia_async::ArrayIterator<'a, i32>>,
    pub bingint_: Option<cornucopia_async::ArrayIterator<'a, i64>>,
    pub int8_: Option<cornucopia_async::ArrayIterator<'a, i64>>,
    pub float4_: Option<cornucopia_async::ArrayIterator<'a, f32>>,
    pub real_: Option<cornucopia_async::ArrayIterator<'a, f32>>,
    pub float8_: Option<cornucopia_async::ArrayIterator<'a, f64>>,
    pub double_precision_: Option<cornucopia_async::ArrayIterator<'a, f64>>,
    pub text_: Option<cornucopia_async::ArrayIterator<'a, &'a str>>,
    pub varchar_: Option<cornucopia_async::ArrayIterator<'a, &'a str>>,
    pub bytea_: Option<cornucopia_async::ArrayIterator<'a, &'a [u8]>>,
    pub timestamp_: Option<cornucopia_async::ArrayIterator<'a, chrono::NaiveDateTime>>,
    pub timestamp_without_time_zone_:
        Option<cornucopia_async::ArrayIterator<'a, chrono::NaiveDateTime>>,
    pub timestamptz_: Option<cornucopia_async::ArrayIterator<'a, chrono::DateTime<chrono::Utc>>>,
    pub timestamp_with_time_zone_:
        Option<cornucopia_async::ArrayIterator<'a, chrono::DateTime<chrono::Utc>>>,
    pub date_: Option<cornucopia_async::ArrayIterator<'a, chrono::NaiveDate>>,
    pub time_: Option<cornucopia_async::ArrayIterator<'a, chrono::NaiveTime>>,
    pub json_: Option<
        cornucopia_async::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>,
    >,
    pub jsonb_: Option<
        cornucopia_async::ArrayIterator<'a, postgres_types::Json<&'a serde_json::value::RawValue>>,
    >,
    pub uuid_: Option<cornucopia_async::ArrayIterator<'a, uuid::Uuid>>,
    pub inet_: Option<cornucopia_async::ArrayIterator<'a, std::net::IpAddr>>,
    pub macaddr_: Option<cornucopia_async::ArrayIterator<'a, eui48::MacAddress>>,
    pub numeric_: Option<cornucopia_async::ArrayIterator<'a, rust_decimal::Decimal>>,
}
impl<'a> From<EverythingArrayNullBorrowed<'a>> for EverythingArrayNull {
    fn from(
        EverythingArrayNullBorrowed {
            bool_,
            boolean_,
            char_,
            smallint_,
            int2_,
            int_,
            int4_,
            bingint_,
            int8_,
            float4_,
            real_,
            float8_,
            double_precision_,
            text_,
            varchar_,
            bytea_,
            timestamp_,
            timestamp_without_time_zone_,
            timestamptz_,
            timestamp_with_time_zone_,
            date_,
            time_,
            json_,
            jsonb_,
            uuid_,
            inet_,
            macaddr_,
            numeric_,
        }: EverythingArrayNullBorrowed<'a>,
    ) -> Self {
        Self {
            bool_: bool_.map(|v| v.map(|v| v).collect()),
            boolean_: boolean_.map(|v| v.map(|v| v).collect()),
            char_: char_.map(|v| v.map(|v| v).collect()),
            smallint_: smallint_.map(|v| v.map(|v| v).collect()),
            int2_: int2_.map(|v| v.map(|v| v).collect()),
            int_: int_.map(|v| v.map(|v| v).collect()),
            int4_: int4_.map(|v| v.map(|v| v).collect()),
            bingint_: bingint_.map(|v| v.map(|v| v).collect()),
            int8_: int8_.map(|v| v.map(|v| v).collect()),
            float4_: float4_.map(|v| v.map(|v| v).collect()),
            real_: real_.map(|v| v.map(|v| v).collect()),
            float8_: float8_.map(|v| v.map(|v| v).collect()),
            double_precision_: double_precision_.map(|v| v.map(|v| v).collect()),
            text_: text_.map(|v| v.map(|v| v.into()).collect()),
            varchar_: varchar_.map(|v| v.map(|v| v.into()).collect()),
            bytea_: bytea_.map(|v| v.map(|v| v.into()).collect()),
            timestamp_: timestamp_.map(|v| v.map(|v| v).collect()),
            timestamp_without_time_zone_: timestamp_without_time_zone_
                .map(|v| v.map(|v| v).collect()),
            timestamptz_: timestamptz_.map(|v| v.map(|v| v).collect()),
            timestamp_with_time_zone_: timestamp_with_time_zone_.map(|v| v.map(|v| v).collect()),
            date_: date_.map(|v| v.map(|v| v).collect()),
            time_: time_.map(|v| v.map(|v| v).collect()),
            json_: json_.map(|v| {
                v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                    .collect()
            }),
            jsonb_: jsonb_.map(|v| {
                v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                    .collect()
            }),
            uuid_: uuid_.map(|v| v.map(|v| v).collect()),
            inet_: inet_.map(|v| v.map(|v| v).collect()),
            macaddr_: macaddr_.map(|v| v.map(|v| v).collect()),
            numeric_: numeric_.map(|v| v.map(|v| v).collect()),
        }
    }
}
pub mod sync {
    use cornucopia_sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct EverythingQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::EverythingBorrowed,
        mapper: fn(super::EverythingBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingBorrowed) -> R,
        ) -> EverythingQuery<'a, C, R, N> {
            EverythingQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row =
                cornucopia_sync::private::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                cornucopia_sync::private::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stream = cornucopia_sync::private::raw(
                self.client,
                self.query,
                cornucopia_sync::private::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(mapped)
        }
    }
    pub struct EverythingNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::EverythingNullBorrowed,
        mapper: fn(super::EverythingNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingNullBorrowed) -> R,
        ) -> EverythingNullQuery<'a, C, R, N> {
            EverythingNullQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row =
                cornucopia_sync::private::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                cornucopia_sync::private::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stream = cornucopia_sync::private::raw(
                self.client,
                self.query,
                cornucopia_sync::private::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(mapped)
        }
    }
    pub struct EverythingArrayQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::EverythingArrayBorrowed,
        mapper: fn(super::EverythingArrayBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingArrayQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayBorrowed) -> R,
        ) -> EverythingArrayQuery<'a, C, R, N> {
            EverythingArrayQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row =
                cornucopia_sync::private::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                cornucopia_sync::private::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stream = cornucopia_sync::private::raw(
                self.client,
                self.query,
                cornucopia_sync::private::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(mapped)
        }
    }
    pub struct EverythingArrayNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::EverythingArrayNullBorrowed,
        mapper: fn(super::EverythingArrayNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingArrayNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayNullBorrowed) -> R,
        ) -> EverythingArrayNullQuery<'a, C, R, N> {
            EverythingArrayNullQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row =
                cornucopia_sync::private::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                cornucopia_sync::private::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stream = cornucopia_sync::private::raw(
                self.client,
                self.query,
                cornucopia_sync::private::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(mapped)
        }
    }
    pub struct PublicNightmareCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor:
            fn(&postgres::Row) -> super::super::super::types::public::NightmareCompositeBorrowed,
        mapper: fn(super::super::super::types::public::NightmareCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> PublicNightmareCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::super::super::types::public::NightmareCompositeBorrowed) -> R,
        ) -> PublicNightmareCompositeQuery<'a, C, R, N> {
            PublicNightmareCompositeQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub fn one(self) -> Result<T, postgres::Error> {
            let row =
                cornucopia_sync::private::one(self.client, self.query, &self.params, self.cached)?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub fn all(self) -> Result<Vec<T>, postgres::Error> {
            self.iter()?.collect()
        }
        pub fn opt(self) -> Result<Option<T>, postgres::Error> {
            let opt_row =
                cornucopia_sync::private::opt(self.client, self.query, &self.params, self.cached)?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub fn iter(
            self,
        ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
        {
            let stream = cornucopia_sync::private::raw(
                self.client,
                self.query,
                cornucopia_sync::private::slice_iter(&self.params),
                self.cached,
            )?;
            let mapped = stream
                .iterator()
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
            Ok(mapped)
        }
    }
    pub fn select_everything() -> SelectEverythingStmt {
        SelectEverythingStmt(
            "SELECT
    *
FROM
    Everything",
            None,
        )
    }
    pub struct SelectEverythingStmt(&'static str, Option<postgres::Statement>);
    impl SelectEverythingStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a mut C,
        ) -> EverythingQuery<'a, C, super::Everything, 0> {
            EverythingQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::EverythingBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    smallserial_: row.get(5),
                    serial2_: row.get(6),
                    int_: row.get(7),
                    int4_: row.get(8),
                    serial_: row.get(9),
                    serial4_: row.get(10),
                    bingint_: row.get(11),
                    int8_: row.get(12),
                    bigserial_: row.get(13),
                    serial8_: row.get(14),
                    float4_: row.get(15),
                    real_: row.get(16),
                    float8_: row.get(17),
                    double_precision_: row.get(18),
                    text_: row.get(19),
                    varchar_: row.get(20),
                    bytea_: row.get(21),
                    timestamp_: row.get(22),
                    timestamp_without_time_zone_: row.get(23),
                    timestamptz_: row.get(24),
                    timestamp_with_time_zone_: row.get(25),
                    date_: row.get(26),
                    time_: row.get(27),
                    json_: row.get(28),
                    jsonb_: row.get(29),
                    uuid_: row.get(30),
                    inet_: row.get(31),
                    macaddr_: row.get(32),
                    numeric_: row.get(33),
                },
                mapper: |it| <super::Everything>::from(it),
            }
        }
    }
    pub fn select_everything_null() -> SelectEverythingNullStmt {
        SelectEverythingNullStmt(
            "SELECT
    *
FROM
    Everything",
            None,
        )
    }
    pub struct SelectEverythingNullStmt(&'static str, Option<postgres::Statement>);
    impl SelectEverythingNullStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a mut C,
        ) -> EverythingNullQuery<'a, C, super::EverythingNull, 0> {
            EverythingNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::EverythingNullBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    smallserial_: row.get(5),
                    serial2_: row.get(6),
                    int_: row.get(7),
                    int4_: row.get(8),
                    serial_: row.get(9),
                    serial4_: row.get(10),
                    bingint_: row.get(11),
                    int8_: row.get(12),
                    bigserial_: row.get(13),
                    serial8_: row.get(14),
                    float4_: row.get(15),
                    real_: row.get(16),
                    float8_: row.get(17),
                    double_precision_: row.get(18),
                    text_: row.get(19),
                    varchar_: row.get(20),
                    bytea_: row.get(21),
                    timestamp_: row.get(22),
                    timestamp_without_time_zone_: row.get(23),
                    timestamptz_: row.get(24),
                    timestamp_with_time_zone_: row.get(25),
                    date_: row.get(26),
                    time_: row.get(27),
                    json_: row.get(28),
                    jsonb_: row.get(29),
                    uuid_: row.get(30),
                    inet_: row.get(31),
                    macaddr_: row.get(32),
                    numeric_: row.get(33),
                },
                mapper: |it| <super::EverythingNull>::from(it),
            }
        }
    }
    pub fn insert_everything() -> InsertEverythingStmt {
        InsertEverythingStmt("INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34)", None)
    }
    pub struct InsertEverythingStmt(&'static str, Option<postgres::Statement>);
    impl InsertEverythingStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<
            'a,
            C: GenericClient,
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::StringSql,
            T3: cornucopia_sync::BytesSql,
            T4: cornucopia_sync::JsonSql,
            T5: cornucopia_sync::JsonSql,
        >(
            &'a self,
            client: &'a mut C,
            bool_: &'a bool,
            boolean_: &'a bool,
            char_: &'a i8,
            smallint_: &'a i16,
            int2_: &'a i16,
            smallserial_: &'a i16,
            serial2_: &'a i16,
            int_: &'a i32,
            int4_: &'a i32,
            serial_: &'a i32,
            serial4_: &'a i32,
            bingint_: &'a i64,
            int8_: &'a i64,
            bigserial_: &'a i64,
            serial8_: &'a i64,
            float4_: &'a f32,
            real_: &'a f32,
            float8_: &'a f64,
            double_precision_: &'a f64,
            text_: &'a T1,
            varchar_: &'a T2,
            bytea_: &'a T3,
            timestamp_: &'a chrono::NaiveDateTime,
            timestamp_without_time_zone_: &'a chrono::NaiveDateTime,
            timestamptz_: &'a chrono::DateTime<chrono::Utc>,
            timestamp_with_time_zone_: &'a chrono::DateTime<chrono::Utc>,
            date_: &'a chrono::NaiveDate,
            time_: &'a chrono::NaiveTime,
            json_: &'a T4,
            jsonb_: &'a T5,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
            numeric_: &'a rust_decimal::Decimal,
        ) -> Result<u64, postgres::Error> {
            client.execute(
                self.0,
                &[
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_,
                    varchar_,
                    bytea_,
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_,
                    jsonb_,
                    uuid_,
                    inet_,
                    macaddr_,
                    numeric_,
                ],
            )
        }
    }
    impl<
            'a,
            C: GenericClient,
            T1: cornucopia_sync::StringSql,
            T2: cornucopia_sync::StringSql,
            T3: cornucopia_sync::BytesSql,
            T4: cornucopia_sync::JsonSql,
            T5: cornucopia_sync::JsonSql,
        >
        cornucopia_sync::Params<
            'a,
            super::EverythingParams<T1, T2, T3, T4, T5>,
            Result<u64, postgres::Error>,
            C,
        > for InsertEverythingStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::EverythingParams<T1, T2, T3, T4, T5>,
        ) -> Result<u64, postgres::Error> {
            self.bind(
                client,
                &params.bool_,
                &params.boolean_,
                &params.char_,
                &params.smallint_,
                &params.int2_,
                &params.smallserial_,
                &params.serial2_,
                &params.int_,
                &params.int4_,
                &params.serial_,
                &params.serial4_,
                &params.bingint_,
                &params.int8_,
                &params.bigserial_,
                &params.serial8_,
                &params.float4_,
                &params.real_,
                &params.float8_,
                &params.double_precision_,
                &params.text_,
                &params.varchar_,
                &params.bytea_,
                &params.timestamp_,
                &params.timestamp_without_time_zone_,
                &params.timestamptz_,
                &params.timestamp_with_time_zone_,
                &params.date_,
                &params.time_,
                &params.json_,
                &params.jsonb_,
                &params.uuid_,
                &params.inet_,
                &params.macaddr_,
                &params.numeric_,
            )
        }
    }
    pub fn select_everything_array() -> SelectEverythingArrayStmt {
        SelectEverythingArrayStmt(
            "SELECT
    *
FROM
    EverythingArray",
            None,
        )
    }
    pub struct SelectEverythingArrayStmt(&'static str, Option<postgres::Statement>);
    impl SelectEverythingArrayStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a mut C,
        ) -> EverythingArrayQuery<'a, C, super::EverythingArray, 0> {
            EverythingArrayQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::EverythingArrayBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    int_: row.get(5),
                    int4_: row.get(6),
                    bingint_: row.get(7),
                    int8_: row.get(8),
                    float4_: row.get(9),
                    real_: row.get(10),
                    float8_: row.get(11),
                    double_precision_: row.get(12),
                    text_: row.get(13),
                    varchar_: row.get(14),
                    bytea_: row.get(15),
                    timestamp_: row.get(16),
                    timestamp_without_time_zone_: row.get(17),
                    timestamptz_: row.get(18),
                    timestamp_with_time_zone_: row.get(19),
                    date_: row.get(20),
                    time_: row.get(21),
                    json_: row.get(22),
                    jsonb_: row.get(23),
                    uuid_: row.get(24),
                    inet_: row.get(25),
                    macaddr_: row.get(26),
                    numeric_: row.get(27),
                },
                mapper: |it| <super::EverythingArray>::from(it),
            }
        }
    }
    pub fn select_everything_array_null() -> SelectEverythingArrayNullStmt {
        SelectEverythingArrayNullStmt(
            "SELECT
    *
FROM
    EverythingArray",
            None,
        )
    }
    pub struct SelectEverythingArrayNullStmt(&'static str, Option<postgres::Statement>);
    impl SelectEverythingArrayNullStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a mut C,
        ) -> EverythingArrayNullQuery<'a, C, super::EverythingArrayNull, 0> {
            EverythingArrayNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::EverythingArrayNullBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    int_: row.get(5),
                    int4_: row.get(6),
                    bingint_: row.get(7),
                    int8_: row.get(8),
                    float4_: row.get(9),
                    real_: row.get(10),
                    float8_: row.get(11),
                    double_precision_: row.get(12),
                    text_: row.get(13),
                    varchar_: row.get(14),
                    bytea_: row.get(15),
                    timestamp_: row.get(16),
                    timestamp_without_time_zone_: row.get(17),
                    timestamptz_: row.get(18),
                    timestamp_with_time_zone_: row.get(19),
                    date_: row.get(20),
                    time_: row.get(21),
                    json_: row.get(22),
                    jsonb_: row.get(23),
                    uuid_: row.get(24),
                    inet_: row.get(25),
                    macaddr_: row.get(26),
                    numeric_: row.get(27),
                },
                mapper: |it| <super::EverythingArrayNull>::from(it),
            }
        }
    }
    pub fn insert_everything_array() -> InsertEverythingArrayStmt {
        InsertEverythingArrayStmt("INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)", None)
    }
    pub struct InsertEverythingArrayStmt(&'static str, Option<postgres::Statement>);
    impl InsertEverythingArrayStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<
            'a,
            C: GenericClient,
            T1: cornucopia_sync::ArraySql<Item = bool>,
            T2: cornucopia_sync::ArraySql<Item = bool>,
            T3: cornucopia_sync::ArraySql<Item = i8>,
            T4: cornucopia_sync::ArraySql<Item = i16>,
            T5: cornucopia_sync::ArraySql<Item = i16>,
            T6: cornucopia_sync::ArraySql<Item = i32>,
            T7: cornucopia_sync::ArraySql<Item = i32>,
            T8: cornucopia_sync::ArraySql<Item = i64>,
            T9: cornucopia_sync::ArraySql<Item = i64>,
            T10: cornucopia_sync::ArraySql<Item = f32>,
            T11: cornucopia_sync::ArraySql<Item = f32>,
            T12: cornucopia_sync::ArraySql<Item = f64>,
            T13: cornucopia_sync::ArraySql<Item = f64>,
            T14: cornucopia_sync::StringSql,
            T15: cornucopia_sync::ArraySql<Item = T14>,
            T16: cornucopia_sync::StringSql,
            T17: cornucopia_sync::ArraySql<Item = T16>,
            T18: cornucopia_sync::BytesSql,
            T19: cornucopia_sync::ArraySql<Item = T18>,
            T20: cornucopia_sync::ArraySql<Item = chrono::NaiveDateTime>,
            T21: cornucopia_sync::ArraySql<Item = chrono::NaiveDateTime>,
            T22: cornucopia_sync::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
            T23: cornucopia_sync::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
            T24: cornucopia_sync::ArraySql<Item = chrono::NaiveDate>,
            T25: cornucopia_sync::ArraySql<Item = chrono::NaiveTime>,
            T26: cornucopia_sync::JsonSql,
            T27: cornucopia_sync::ArraySql<Item = T26>,
            T28: cornucopia_sync::JsonSql,
            T29: cornucopia_sync::ArraySql<Item = T28>,
            T30: cornucopia_sync::ArraySql<Item = uuid::Uuid>,
            T31: cornucopia_sync::ArraySql<Item = std::net::IpAddr>,
            T32: cornucopia_sync::ArraySql<Item = eui48::MacAddress>,
            T33: cornucopia_sync::ArraySql<Item = rust_decimal::Decimal>,
        >(
            &'a self,
            client: &'a mut C,
            bool_: &'a T1,
            boolean_: &'a T2,
            char_: &'a T3,
            smallint_: &'a T4,
            int2_: &'a T5,
            int_: &'a T6,
            int4_: &'a T7,
            bingint_: &'a T8,
            int8_: &'a T9,
            float4_: &'a T10,
            real_: &'a T11,
            float8_: &'a T12,
            double_precision_: &'a T13,
            text_: &'a T15,
            varchar_: &'a T17,
            bytea_: &'a T19,
            timestamp_: &'a T20,
            timestamp_without_time_zone_: &'a T21,
            timestamptz_: &'a T22,
            timestamp_with_time_zone_: &'a T23,
            date_: &'a T24,
            time_: &'a T25,
            json_: &'a T27,
            jsonb_: &'a T29,
            uuid_: &'a T30,
            inet_: &'a T31,
            macaddr_: &'a T32,
            numeric_: &'a T33,
        ) -> Result<u64, postgres::Error> {
            client.execute(
                self.0,
                &[
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    int_,
                    int4_,
                    bingint_,
                    int8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_,
                    varchar_,
                    bytea_,
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_,
                    jsonb_,
                    uuid_,
                    inet_,
                    macaddr_,
                    numeric_,
                ],
            )
        }
    }
    impl<
            'a,
            C: GenericClient,
            T1: cornucopia_sync::ArraySql<Item = bool>,
            T2: cornucopia_sync::ArraySql<Item = bool>,
            T3: cornucopia_sync::ArraySql<Item = i8>,
            T4: cornucopia_sync::ArraySql<Item = i16>,
            T5: cornucopia_sync::ArraySql<Item = i16>,
            T6: cornucopia_sync::ArraySql<Item = i32>,
            T7: cornucopia_sync::ArraySql<Item = i32>,
            T8: cornucopia_sync::ArraySql<Item = i64>,
            T9: cornucopia_sync::ArraySql<Item = i64>,
            T10: cornucopia_sync::ArraySql<Item = f32>,
            T11: cornucopia_sync::ArraySql<Item = f32>,
            T12: cornucopia_sync::ArraySql<Item = f64>,
            T13: cornucopia_sync::ArraySql<Item = f64>,
            T14: cornucopia_sync::StringSql,
            T15: cornucopia_sync::ArraySql<Item = T14>,
            T16: cornucopia_sync::StringSql,
            T17: cornucopia_sync::ArraySql<Item = T16>,
            T18: cornucopia_sync::BytesSql,
            T19: cornucopia_sync::ArraySql<Item = T18>,
            T20: cornucopia_sync::ArraySql<Item = chrono::NaiveDateTime>,
            T21: cornucopia_sync::ArraySql<Item = chrono::NaiveDateTime>,
            T22: cornucopia_sync::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
            T23: cornucopia_sync::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
            T24: cornucopia_sync::ArraySql<Item = chrono::NaiveDate>,
            T25: cornucopia_sync::ArraySql<Item = chrono::NaiveTime>,
            T26: cornucopia_sync::JsonSql,
            T27: cornucopia_sync::ArraySql<Item = T26>,
            T28: cornucopia_sync::JsonSql,
            T29: cornucopia_sync::ArraySql<Item = T28>,
            T30: cornucopia_sync::ArraySql<Item = uuid::Uuid>,
            T31: cornucopia_sync::ArraySql<Item = std::net::IpAddr>,
            T32: cornucopia_sync::ArraySql<Item = eui48::MacAddress>,
            T33: cornucopia_sync::ArraySql<Item = rust_decimal::Decimal>,
        >
        cornucopia_sync::Params<
            'a,
            super::EverythingArrayParams<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
                T29,
                T30,
                T31,
                T32,
                T33,
            >,
            Result<u64, postgres::Error>,
            C,
        > for InsertEverythingArrayStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::EverythingArrayParams<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
                T29,
                T30,
                T31,
                T32,
                T33,
            >,
        ) -> Result<u64, postgres::Error> {
            self.bind(
                client,
                &params.bool_,
                &params.boolean_,
                &params.char_,
                &params.smallint_,
                &params.int2_,
                &params.int_,
                &params.int4_,
                &params.bingint_,
                &params.int8_,
                &params.float4_,
                &params.real_,
                &params.float8_,
                &params.double_precision_,
                &params.text_,
                &params.varchar_,
                &params.bytea_,
                &params.timestamp_,
                &params.timestamp_without_time_zone_,
                &params.timestamptz_,
                &params.timestamp_with_time_zone_,
                &params.date_,
                &params.time_,
                &params.json_,
                &params.jsonb_,
                &params.uuid_,
                &params.inet_,
                &params.macaddr_,
                &params.numeric_,
            )
        }
    }
    pub fn select_nightmare() -> SelectNightmareStmt {
        SelectNightmareStmt(
            "SELECT
    *
FROM
    nightmare",
            None,
        )
    }
    pub struct SelectNightmareStmt(&'static str, Option<postgres::Statement>);
    impl SelectNightmareStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a mut C,
        ) -> PublicNightmareCompositeQuery<
            'a,
            C,
            super::super::super::types::public::NightmareComposite,
            0,
        > {
            PublicNightmareCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn insert_nightmare() -> InsertNightmareStmt {
        InsertNightmareStmt(
            "INSERT INTO nightmare (composite)
    VALUES ($1)",
            None,
        )
    }
    pub struct InsertNightmareStmt(&'static str, Option<postgres::Statement>);
    impl InsertNightmareStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a mut C,
            composite: &'a super::super::super::types::public::NightmareCompositeParams<'a>,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[composite])
        }
    }
}
pub mod async_ {
    use cornucopia_async::GenericClient;
    use futures_util::{StreamExt, TryStreamExt};
    pub struct EverythingQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::EverythingBorrowed,
        mapper: fn(super::EverythingBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingBorrowed) -> R,
        ) -> EverythingQuery<'a, C, R, N> {
            EverythingQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                cornucopia_async::private::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                cornucopia_async::private::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures_util::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stream = cornucopia_async::private::raw(
                self.client,
                self.query,
                cornucopia_async::private::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct EverythingNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::EverythingNullBorrowed,
        mapper: fn(super::EverythingNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingNullBorrowed) -> R,
        ) -> EverythingNullQuery<'a, C, R, N> {
            EverythingNullQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                cornucopia_async::private::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                cornucopia_async::private::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures_util::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stream = cornucopia_async::private::raw(
                self.client,
                self.query,
                cornucopia_async::private::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct EverythingArrayQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::EverythingArrayBorrowed,
        mapper: fn(super::EverythingArrayBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingArrayQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayBorrowed) -> R,
        ) -> EverythingArrayQuery<'a, C, R, N> {
            EverythingArrayQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                cornucopia_async::private::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                cornucopia_async::private::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures_util::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stream = cornucopia_async::private::raw(
                self.client,
                self.query,
                cornucopia_async::private::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct EverythingArrayNullQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::EverythingArrayNullBorrowed,
        mapper: fn(super::EverythingArrayNullBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> EverythingArrayNullQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::EverythingArrayNullBorrowed) -> R,
        ) -> EverythingArrayNullQuery<'a, C, R, N> {
            EverythingArrayNullQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                cornucopia_async::private::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                cornucopia_async::private::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures_util::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stream = cornucopia_async::private::raw(
                self.client,
                self.query,
                cornucopia_async::private::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(mapped)
        }
    }
    pub struct PublicNightmareCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(
            &tokio_postgres::Row,
        ) -> super::super::super::types::public::NightmareCompositeBorrowed,
        mapper: fn(super::super::super::types::public::NightmareCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> PublicNightmareCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::super::super::types::public::NightmareCompositeBorrowed) -> R,
        ) -> PublicNightmareCompositeQuery<'a, C, R, N> {
            PublicNightmareCompositeQuery {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }
        pub async fn one(self) -> Result<T, tokio_postgres::Error> {
            let row =
                cornucopia_async::private::one(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }
        pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
            self.iter().await?.try_collect().await
        }
        pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
            let opt_row =
                cornucopia_async::private::opt(self.client, self.query, &self.params, self.cached)
                    .await?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }
        pub async fn iter(
            self,
        ) -> Result<
            impl futures_util::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
            tokio_postgres::Error,
        > {
            let stream = cornucopia_async::private::raw(
                self.client,
                self.query,
                cornucopia_async::private::slice_iter(&self.params),
                self.cached,
            )
            .await?;
            let mapped = stream
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                .into_stream();
            Ok(mapped)
        }
    }
    pub fn select_everything() -> SelectEverythingStmt {
        SelectEverythingStmt(
            "SELECT
    *
FROM
    Everything",
            None,
        )
    }
    pub struct SelectEverythingStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectEverythingStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a C,
        ) -> EverythingQuery<'a, C, super::Everything, 0> {
            EverythingQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::EverythingBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    smallserial_: row.get(5),
                    serial2_: row.get(6),
                    int_: row.get(7),
                    int4_: row.get(8),
                    serial_: row.get(9),
                    serial4_: row.get(10),
                    bingint_: row.get(11),
                    int8_: row.get(12),
                    bigserial_: row.get(13),
                    serial8_: row.get(14),
                    float4_: row.get(15),
                    real_: row.get(16),
                    float8_: row.get(17),
                    double_precision_: row.get(18),
                    text_: row.get(19),
                    varchar_: row.get(20),
                    bytea_: row.get(21),
                    timestamp_: row.get(22),
                    timestamp_without_time_zone_: row.get(23),
                    timestamptz_: row.get(24),
                    timestamp_with_time_zone_: row.get(25),
                    date_: row.get(26),
                    time_: row.get(27),
                    json_: row.get(28),
                    jsonb_: row.get(29),
                    uuid_: row.get(30),
                    inet_: row.get(31),
                    macaddr_: row.get(32),
                    numeric_: row.get(33),
                },
                mapper: |it| <super::Everything>::from(it),
            }
        }
    }
    pub fn select_everything_null() -> SelectEverythingNullStmt {
        SelectEverythingNullStmt(
            "SELECT
    *
FROM
    Everything",
            None,
        )
    }
    pub struct SelectEverythingNullStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectEverythingNullStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a C,
        ) -> EverythingNullQuery<'a, C, super::EverythingNull, 0> {
            EverythingNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::EverythingNullBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    smallserial_: row.get(5),
                    serial2_: row.get(6),
                    int_: row.get(7),
                    int4_: row.get(8),
                    serial_: row.get(9),
                    serial4_: row.get(10),
                    bingint_: row.get(11),
                    int8_: row.get(12),
                    bigserial_: row.get(13),
                    serial8_: row.get(14),
                    float4_: row.get(15),
                    real_: row.get(16),
                    float8_: row.get(17),
                    double_precision_: row.get(18),
                    text_: row.get(19),
                    varchar_: row.get(20),
                    bytea_: row.get(21),
                    timestamp_: row.get(22),
                    timestamp_without_time_zone_: row.get(23),
                    timestamptz_: row.get(24),
                    timestamp_with_time_zone_: row.get(25),
                    date_: row.get(26),
                    time_: row.get(27),
                    json_: row.get(28),
                    jsonb_: row.get(29),
                    uuid_: row.get(30),
                    inet_: row.get(31),
                    macaddr_: row.get(32),
                    numeric_: row.get(33),
                },
                mapper: |it| <super::EverythingNull>::from(it),
            }
        }
    }
    pub fn insert_everything() -> InsertEverythingStmt {
        InsertEverythingStmt("INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34)", None)
    }
    pub struct InsertEverythingStmt(&'static str, Option<tokio_postgres::Statement>);
    impl InsertEverythingStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<
            'a,
            C: GenericClient,
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::BytesSql,
            T4: cornucopia_async::JsonSql,
            T5: cornucopia_async::JsonSql,
        >(
            &'a self,
            client: &'a C,
            bool_: &'a bool,
            boolean_: &'a bool,
            char_: &'a i8,
            smallint_: &'a i16,
            int2_: &'a i16,
            smallserial_: &'a i16,
            serial2_: &'a i16,
            int_: &'a i32,
            int4_: &'a i32,
            serial_: &'a i32,
            serial4_: &'a i32,
            bingint_: &'a i64,
            int8_: &'a i64,
            bigserial_: &'a i64,
            serial8_: &'a i64,
            float4_: &'a f32,
            real_: &'a f32,
            float8_: &'a f64,
            double_precision_: &'a f64,
            text_: &'a T1,
            varchar_: &'a T2,
            bytea_: &'a T3,
            timestamp_: &'a chrono::NaiveDateTime,
            timestamp_without_time_zone_: &'a chrono::NaiveDateTime,
            timestamptz_: &'a chrono::DateTime<chrono::Utc>,
            timestamp_with_time_zone_: &'a chrono::DateTime<chrono::Utc>,
            date_: &'a chrono::NaiveDate,
            time_: &'a chrono::NaiveTime,
            json_: &'a T4,
            jsonb_: &'a T5,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
            numeric_: &'a rust_decimal::Decimal,
        ) -> Result<u64, tokio_postgres::Error> {
            client
                .execute(
                    self.0,
                    &[
                        bool_,
                        boolean_,
                        char_,
                        smallint_,
                        int2_,
                        smallserial_,
                        serial2_,
                        int_,
                        int4_,
                        serial_,
                        serial4_,
                        bingint_,
                        int8_,
                        bigserial_,
                        serial8_,
                        float4_,
                        real_,
                        float8_,
                        double_precision_,
                        text_,
                        varchar_,
                        bytea_,
                        timestamp_,
                        timestamp_without_time_zone_,
                        timestamptz_,
                        timestamp_with_time_zone_,
                        date_,
                        time_,
                        json_,
                        jsonb_,
                        uuid_,
                        inet_,
                        macaddr_,
                        numeric_,
                    ],
                )
                .await
        }
    }
    impl<
            'a,
            C: GenericClient + Send + Sync,
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::BytesSql,
            T4: cornucopia_async::JsonSql,
            T5: cornucopia_async::JsonSql,
        >
        cornucopia_async::Params<
            'a,
            super::EverythingParams<T1, T2, T3, T4, T5>,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for InsertEverythingStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::EverythingParams<T1, T2, T3, T4, T5>,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(
                client,
                &params.bool_,
                &params.boolean_,
                &params.char_,
                &params.smallint_,
                &params.int2_,
                &params.smallserial_,
                &params.serial2_,
                &params.int_,
                &params.int4_,
                &params.serial_,
                &params.serial4_,
                &params.bingint_,
                &params.int8_,
                &params.bigserial_,
                &params.serial8_,
                &params.float4_,
                &params.real_,
                &params.float8_,
                &params.double_precision_,
                &params.text_,
                &params.varchar_,
                &params.bytea_,
                &params.timestamp_,
                &params.timestamp_without_time_zone_,
                &params.timestamptz_,
                &params.timestamp_with_time_zone_,
                &params.date_,
                &params.time_,
                &params.json_,
                &params.jsonb_,
                &params.uuid_,
                &params.inet_,
                &params.macaddr_,
                &params.numeric_,
            ))
        }
    }
    pub fn select_everything_array() -> SelectEverythingArrayStmt {
        SelectEverythingArrayStmt(
            "SELECT
    *
FROM
    EverythingArray",
            None,
        )
    }
    pub struct SelectEverythingArrayStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectEverythingArrayStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a C,
        ) -> EverythingArrayQuery<'a, C, super::EverythingArray, 0> {
            EverythingArrayQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::EverythingArrayBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    int_: row.get(5),
                    int4_: row.get(6),
                    bingint_: row.get(7),
                    int8_: row.get(8),
                    float4_: row.get(9),
                    real_: row.get(10),
                    float8_: row.get(11),
                    double_precision_: row.get(12),
                    text_: row.get(13),
                    varchar_: row.get(14),
                    bytea_: row.get(15),
                    timestamp_: row.get(16),
                    timestamp_without_time_zone_: row.get(17),
                    timestamptz_: row.get(18),
                    timestamp_with_time_zone_: row.get(19),
                    date_: row.get(20),
                    time_: row.get(21),
                    json_: row.get(22),
                    jsonb_: row.get(23),
                    uuid_: row.get(24),
                    inet_: row.get(25),
                    macaddr_: row.get(26),
                    numeric_: row.get(27),
                },
                mapper: |it| <super::EverythingArray>::from(it),
            }
        }
    }
    pub fn select_everything_array_null() -> SelectEverythingArrayNullStmt {
        SelectEverythingArrayNullStmt(
            "SELECT
    *
FROM
    EverythingArray",
            None,
        )
    }
    pub struct SelectEverythingArrayNullStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectEverythingArrayNullStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a C,
        ) -> EverythingArrayNullQuery<'a, C, super::EverythingArrayNull, 0> {
            EverythingArrayNullQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::EverythingArrayNullBorrowed {
                    bool_: row.get(0),
                    boolean_: row.get(1),
                    char_: row.get(2),
                    smallint_: row.get(3),
                    int2_: row.get(4),
                    int_: row.get(5),
                    int4_: row.get(6),
                    bingint_: row.get(7),
                    int8_: row.get(8),
                    float4_: row.get(9),
                    real_: row.get(10),
                    float8_: row.get(11),
                    double_precision_: row.get(12),
                    text_: row.get(13),
                    varchar_: row.get(14),
                    bytea_: row.get(15),
                    timestamp_: row.get(16),
                    timestamp_without_time_zone_: row.get(17),
                    timestamptz_: row.get(18),
                    timestamp_with_time_zone_: row.get(19),
                    date_: row.get(20),
                    time_: row.get(21),
                    json_: row.get(22),
                    jsonb_: row.get(23),
                    uuid_: row.get(24),
                    inet_: row.get(25),
                    macaddr_: row.get(26),
                    numeric_: row.get(27),
                },
                mapper: |it| <super::EverythingArrayNull>::from(it),
            }
        }
    }
    pub fn insert_everything_array() -> InsertEverythingArrayStmt {
        InsertEverythingArrayStmt("INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)", None)
    }
    pub struct InsertEverythingArrayStmt(&'static str, Option<tokio_postgres::Statement>);
    impl InsertEverythingArrayStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<
            'a,
            C: GenericClient,
            T1: cornucopia_async::ArraySql<Item = bool>,
            T2: cornucopia_async::ArraySql<Item = bool>,
            T3: cornucopia_async::ArraySql<Item = i8>,
            T4: cornucopia_async::ArraySql<Item = i16>,
            T5: cornucopia_async::ArraySql<Item = i16>,
            T6: cornucopia_async::ArraySql<Item = i32>,
            T7: cornucopia_async::ArraySql<Item = i32>,
            T8: cornucopia_async::ArraySql<Item = i64>,
            T9: cornucopia_async::ArraySql<Item = i64>,
            T10: cornucopia_async::ArraySql<Item = f32>,
            T11: cornucopia_async::ArraySql<Item = f32>,
            T12: cornucopia_async::ArraySql<Item = f64>,
            T13: cornucopia_async::ArraySql<Item = f64>,
            T14: cornucopia_async::StringSql,
            T15: cornucopia_async::ArraySql<Item = T14>,
            T16: cornucopia_async::StringSql,
            T17: cornucopia_async::ArraySql<Item = T16>,
            T18: cornucopia_async::BytesSql,
            T19: cornucopia_async::ArraySql<Item = T18>,
            T20: cornucopia_async::ArraySql<Item = chrono::NaiveDateTime>,
            T21: cornucopia_async::ArraySql<Item = chrono::NaiveDateTime>,
            T22: cornucopia_async::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
            T23: cornucopia_async::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
            T24: cornucopia_async::ArraySql<Item = chrono::NaiveDate>,
            T25: cornucopia_async::ArraySql<Item = chrono::NaiveTime>,
            T26: cornucopia_async::JsonSql,
            T27: cornucopia_async::ArraySql<Item = T26>,
            T28: cornucopia_async::JsonSql,
            T29: cornucopia_async::ArraySql<Item = T28>,
            T30: cornucopia_async::ArraySql<Item = uuid::Uuid>,
            T31: cornucopia_async::ArraySql<Item = std::net::IpAddr>,
            T32: cornucopia_async::ArraySql<Item = eui48::MacAddress>,
            T33: cornucopia_async::ArraySql<Item = rust_decimal::Decimal>,
        >(
            &'a self,
            client: &'a C,
            bool_: &'a T1,
            boolean_: &'a T2,
            char_: &'a T3,
            smallint_: &'a T4,
            int2_: &'a T5,
            int_: &'a T6,
            int4_: &'a T7,
            bingint_: &'a T8,
            int8_: &'a T9,
            float4_: &'a T10,
            real_: &'a T11,
            float8_: &'a T12,
            double_precision_: &'a T13,
            text_: &'a T15,
            varchar_: &'a T17,
            bytea_: &'a T19,
            timestamp_: &'a T20,
            timestamp_without_time_zone_: &'a T21,
            timestamptz_: &'a T22,
            timestamp_with_time_zone_: &'a T23,
            date_: &'a T24,
            time_: &'a T25,
            json_: &'a T27,
            jsonb_: &'a T29,
            uuid_: &'a T30,
            inet_: &'a T31,
            macaddr_: &'a T32,
            numeric_: &'a T33,
        ) -> Result<u64, tokio_postgres::Error> {
            client
                .execute(
                    self.0,
                    &[
                        bool_,
                        boolean_,
                        char_,
                        smallint_,
                        int2_,
                        int_,
                        int4_,
                        bingint_,
                        int8_,
                        float4_,
                        real_,
                        float8_,
                        double_precision_,
                        text_,
                        varchar_,
                        bytea_,
                        timestamp_,
                        timestamp_without_time_zone_,
                        timestamptz_,
                        timestamp_with_time_zone_,
                        date_,
                        time_,
                        json_,
                        jsonb_,
                        uuid_,
                        inet_,
                        macaddr_,
                        numeric_,
                    ],
                )
                .await
        }
    }
    impl<
            'a,
            C: GenericClient + Send + Sync,
            T1: cornucopia_async::ArraySql<Item = bool>,
            T2: cornucopia_async::ArraySql<Item = bool>,
            T3: cornucopia_async::ArraySql<Item = i8>,
            T4: cornucopia_async::ArraySql<Item = i16>,
            T5: cornucopia_async::ArraySql<Item = i16>,
            T6: cornucopia_async::ArraySql<Item = i32>,
            T7: cornucopia_async::ArraySql<Item = i32>,
            T8: cornucopia_async::ArraySql<Item = i64>,
            T9: cornucopia_async::ArraySql<Item = i64>,
            T10: cornucopia_async::ArraySql<Item = f32>,
            T11: cornucopia_async::ArraySql<Item = f32>,
            T12: cornucopia_async::ArraySql<Item = f64>,
            T13: cornucopia_async::ArraySql<Item = f64>,
            T14: cornucopia_async::StringSql,
            T15: cornucopia_async::ArraySql<Item = T14>,
            T16: cornucopia_async::StringSql,
            T17: cornucopia_async::ArraySql<Item = T16>,
            T18: cornucopia_async::BytesSql,
            T19: cornucopia_async::ArraySql<Item = T18>,
            T20: cornucopia_async::ArraySql<Item = chrono::NaiveDateTime>,
            T21: cornucopia_async::ArraySql<Item = chrono::NaiveDateTime>,
            T22: cornucopia_async::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
            T23: cornucopia_async::ArraySql<Item = chrono::DateTime<chrono::Utc>>,
            T24: cornucopia_async::ArraySql<Item = chrono::NaiveDate>,
            T25: cornucopia_async::ArraySql<Item = chrono::NaiveTime>,
            T26: cornucopia_async::JsonSql,
            T27: cornucopia_async::ArraySql<Item = T26>,
            T28: cornucopia_async::JsonSql,
            T29: cornucopia_async::ArraySql<Item = T28>,
            T30: cornucopia_async::ArraySql<Item = uuid::Uuid>,
            T31: cornucopia_async::ArraySql<Item = std::net::IpAddr>,
            T32: cornucopia_async::ArraySql<Item = eui48::MacAddress>,
            T33: cornucopia_async::ArraySql<Item = rust_decimal::Decimal>,
        >
        cornucopia_async::Params<
            'a,
            super::EverythingArrayParams<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
                T29,
                T30,
                T31,
                T32,
                T33,
            >,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for InsertEverythingArrayStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::EverythingArrayParams<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
                T21,
                T22,
                T23,
                T24,
                T25,
                T26,
                T27,
                T28,
                T29,
                T30,
                T31,
                T32,
                T33,
            >,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(
                client,
                &params.bool_,
                &params.boolean_,
                &params.char_,
                &params.smallint_,
                &params.int2_,
                &params.int_,
                &params.int4_,
                &params.bingint_,
                &params.int8_,
                &params.float4_,
                &params.real_,
                &params.float8_,
                &params.double_precision_,
                &params.text_,
                &params.varchar_,
                &params.bytea_,
                &params.timestamp_,
                &params.timestamp_without_time_zone_,
                &params.timestamptz_,
                &params.timestamp_with_time_zone_,
                &params.date_,
                &params.time_,
                &params.json_,
                &params.jsonb_,
                &params.uuid_,
                &params.inet_,
                &params.macaddr_,
                &params.numeric_,
            ))
        }
    }
    pub fn select_nightmare() -> SelectNightmareStmt {
        SelectNightmareStmt(
            "SELECT
    *
FROM
    nightmare",
            None,
        )
    }
    pub struct SelectNightmareStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectNightmareStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a C,
        ) -> PublicNightmareCompositeQuery<
            'a,
            C,
            super::super::super::types::public::NightmareComposite,
            0,
        > {
            PublicNightmareCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn insert_nightmare() -> InsertNightmareStmt {
        InsertNightmareStmt(
            "INSERT INTO nightmare (composite)
    VALUES ($1)",
            None,
        )
    }
    pub struct InsertNightmareStmt(&'static str, Option<tokio_postgres::Statement>);
    impl InsertNightmareStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub async fn bind<'a, C: GenericClient>(
            &'a self,
            client: &'a C,
            composite: &'a super::super::super::types::public::NightmareCompositeParams<'a>,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[composite]).await
        }
    }
}
