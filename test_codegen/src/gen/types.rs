// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod public {
    use std::str::FromStr;
    #[derive(
        serde::Deserialize, serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq,
    )]
    #[postgres(name = "clone_composite")]
    pub struct CloneComposite {
        #[postgres(name = "first")]
        pub first: i32,
        #[postgres(name = "second")]
        pub second: String,
    }
    #[derive(Debug)]
    pub struct CloneCompositeBorrowed<'a> {
        pub first: i32,
        pub second: &'a str,
    }
    impl<'a> From<CloneCompositeBorrowed<'a>> for CloneComposite {
        fn from(CloneCompositeBorrowed { first, second }: CloneCompositeBorrowed<'a>) -> Self {
            Self {
                first,
                second: second.into(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for CloneCompositeBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<CloneCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let first = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let second = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            Ok(CloneCompositeBorrowed { first, second })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "clone_composite" && ty.schema() == "public"
        }
    }
    impl<'a> postgres_types::ToSql for CloneCompositeBorrowed<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let CloneCompositeBorrowed { first, second } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "first" => postgres_types::ToSql::to_sql(first, field.type_(), out),
                    "second" => postgres_types::ToSql::to_sql(second, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::max_value() as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "clone_composite" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name() {
                        "first" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                        "second" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                        _ => false,
                    })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(
        serde::Deserialize, serde::Serialize, Debug, postgres_types::FromSql, Copy, Clone, PartialEq,
    )]
    #[postgres(name = "copy_composite")]
    pub struct CopyComposite {
        #[postgres(name = "first")]
        pub first: i32,
        #[postgres(name = "second")]
        pub second: f64,
    }
    impl<'a> postgres_types::ToSql for CopyComposite {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let CopyComposite { first, second } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "first" => postgres_types::ToSql::to_sql(first, field.type_(), out),
                    "second" => postgres_types::ToSql::to_sql(second, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::max_value() as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "copy_composite" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name() {
                        "first" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                        "second" => <f64 as postgres_types::ToSql>::accepts(f.type_()),
                        _ => false,
                    })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(
        serde::Deserialize, serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq,
    )]
    #[postgres(name = "domain_composite")]
    pub struct DomainComposite {
        #[postgres(name = "txt")]
        pub txt: String,
        #[postgres(name = "json")]
        pub json: serde_json::Value,
        #[postgres(name = "nb")]
        pub nb: i32,
        #[postgres(name = "arr")]
        pub arr: Vec<serde_json::Value>,
    }
    #[derive(Debug)]
    pub struct DomainCompositeBorrowed<'a> {
        pub txt: &'a str,
        pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
        pub nb: i32,
        pub arr: cornucopia_async::ArrayIterator<
            'a,
            postgres_types::Json<&'a serde_json::value::RawValue>,
        >,
    }
    impl<'a> From<DomainCompositeBorrowed<'a>> for DomainComposite {
        fn from(
            DomainCompositeBorrowed { txt, json, nb, arr }: DomainCompositeBorrowed<'a>,
        ) -> Self {
            Self {
                txt: txt.into(),
                json: serde_json::from_str(json.0.get()).unwrap(),
                nb,
                arr: arr
                    .map(|v| serde_json::from_str(v.0.get()).unwrap())
                    .collect(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for DomainCompositeBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<DomainCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let txt = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let json = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let nb = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let arr = postgres_types::private::read_value(fields[3].type_(), &mut out)?;
            Ok(DomainCompositeBorrowed { txt, json, nb, arr })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "domain_composite" && ty.schema() == "public"
        }
    }
    #[derive(Debug)]
    pub struct DomainCompositeParams<'a> {
        pub txt: &'a str,
        pub json: &'a serde_json::value::Value,
        pub nb: i32,
        pub arr: &'a [&'a serde_json::value::Value],
    }
    impl<'a> postgres_types::ToSql for DomainCompositeParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let DomainCompositeParams { txt, json, nb, arr } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "txt" => postgres_types::ToSql::to_sql(
                        &cornucopia_async::private::Domain(txt),
                        field.type_(),
                        out,
                    ),
                    "json" => postgres_types::ToSql::to_sql(
                        &cornucopia_async::private::Domain(json),
                        field.type_(),
                        out,
                    ),
                    "nb" => postgres_types::ToSql::to_sql(
                        &cornucopia_async::private::Domain(nb),
                        field.type_(),
                        out,
                    ),
                    "arr" => postgres_types::ToSql::to_sql(
                        &cornucopia_async::private::Domain(
                            &cornucopia_async::private::DomainArray(arr),
                        ),
                        field.type_(),
                        out,
                    ),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::max_value() as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "domain_composite" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 4 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name()
                {
                    "txt" => <cornucopia_async::private::Domain::<&'a str> as
                    postgres_types::ToSql>::accepts(f.type_()),"json" => <cornucopia_async::private::Domain::<&'a serde_json::value::Value> as
                    postgres_types::ToSql>::accepts(f.type_()),"nb" => <cornucopia_async::private::Domain::<i32> as
                    postgres_types::ToSql>::accepts(f.type_()),"arr" => <cornucopia_async::private::Domain::<cornucopia_async::private::DomainArray::<&'a serde_json::value::Value, &[&'a serde_json::value::Value]>> as
                    postgres_types::ToSql>::accepts(f.type_()),_ => false,
                })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(
        serde::Deserialize, serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq,
    )]
    #[postgres(name = "named_composite")]
    pub struct NamedComposite {
        #[postgres(name = "wow")]
        pub wow: Option<String>,
        #[postgres(name = "such_cool")]
        pub such_cool: Option<i32>,
    }
    #[derive(Debug)]
    pub struct NamedCompositeBorrowed<'a> {
        pub wow: Option<&'a str>,
        pub such_cool: Option<i32>,
    }
    impl<'a> From<NamedCompositeBorrowed<'a>> for NamedComposite {
        fn from(NamedCompositeBorrowed { wow, such_cool }: NamedCompositeBorrowed<'a>) -> Self {
            Self {
                wow: wow.map(|v| v.into()),
                such_cool,
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for NamedCompositeBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<NamedCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let wow = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            Ok(NamedCompositeBorrowed { wow, such_cool })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "named_composite" && ty.schema() == "public"
        }
    }
    impl<'a> postgres_types::ToSql for NamedCompositeBorrowed<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let NamedCompositeBorrowed { wow, such_cool } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "wow" => postgres_types::ToSql::to_sql(wow, field.type_(), out),
                    "such_cool" => postgres_types::ToSql::to_sql(such_cool, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::max_value() as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "named_composite" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name() {
                        "wow" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                        "such_cool" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                        _ => false,
                    })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(
        serde::Deserialize,
        serde::Serialize,
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        strum::EnumString,
        strum::AsRefStr,
        std::hash::Hash,
        num_derive::FromPrimitive,
    )]
    #[allow(non_camel_case_types)]
    pub enum EnumWithDot {
        #[strum(serialize = "variant.with_dot")]
        variant_with_dot,
    }
    impl<'a> postgres_types::ToSql for EnumWithDot {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            buf: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            buf.extend_from_slice(self.as_ref().as_bytes());
            std::result::Result::Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "enum.with_dot" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Enum(ref variants) => {
                    if variants.len() != 1 {
                        return false;
                    }
                    variants.iter().all(|v| Self::from_str(&**v).is_ok())
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    impl<'a> postgres_types::FromSql<'a> for EnumWithDot {
        fn from_sql(
            ty: &postgres_types::Type,
            buf: &'a [u8],
        ) -> Result<EnumWithDot, Box<dyn std::error::Error + Sync + Send>> {
            let s = std::str::from_utf8(buf)?;
            Ok(Self::from_str(s).map_err(|e| format!("invalid variant `{}`", s))?)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "enum.with_dot" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Enum(ref variants) => {
                    if variants.len() != 1 {
                        return false;
                    }
                    variants.iter().all(|v| Self::from_str(&**v).is_ok())
                }
                _ => false,
            }
        }
    }
    #[derive(
        serde::Deserialize, serde::Serialize, Debug, postgres_types::FromSql, Copy, Clone, PartialEq,
    )]
    #[postgres(name = "named_composite.with_dot")]
    pub struct NamedCompositeWithDot {
        #[postgres(name = "this.is.inconceivable")]
        pub this_is_inconceivable: Option<super::public::EnumWithDot>,
    }
    impl<'a> postgres_types::ToSql for NamedCompositeWithDot {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let NamedCompositeWithDot {
                this_is_inconceivable,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "this.is.inconceivable" => {
                        postgres_types::ToSql::to_sql(this_is_inconceivable, field.type_(), out)
                    }
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::max_value() as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "named_composite.with_dot" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 1 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name() {
                        "this.is.inconceivable" => {
                            <super::public::EnumWithDot as postgres_types::ToSql>::accepts(
                                f.type_(),
                            )
                        }
                        _ => false,
                    })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(
        serde::Deserialize, serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq,
    )]
    #[postgres(name = "nullity_composite")]
    pub struct NullityComposite {
        #[postgres(name = "jsons")]
        pub jsons: Option<Vec<Option<serde_json::Value>>>,
        #[postgres(name = "id")]
        pub id: i32,
    }
    #[derive(Debug)]
    pub struct NullityCompositeBorrowed<'a> {
        pub jsons: Option<
            cornucopia_async::ArrayIterator<
                'a,
                Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            >,
        >,
        pub id: i32,
    }
    impl<'a> From<NullityCompositeBorrowed<'a>> for NullityComposite {
        fn from(NullityCompositeBorrowed { jsons, id }: NullityCompositeBorrowed<'a>) -> Self {
            Self {
                jsons: jsons.map(|v| {
                    v.map(|v| v.map(|v| serde_json::from_str(v.0.get()).unwrap()))
                        .collect()
                }),
                id,
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for NullityCompositeBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<NullityCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
        {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let jsons = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let id = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            Ok(NullityCompositeBorrowed { jsons, id })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "nullity_composite" && ty.schema() == "public"
        }
    }
    #[derive(Debug)]
    pub struct NullityCompositeParams<'a> {
        pub jsons: Option<&'a [Option<&'a serde_json::value::Value>]>,
        pub id: i32,
    }
    impl<'a> postgres_types::ToSql for NullityCompositeParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let NullityCompositeParams { jsons, id } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "jsons" => postgres_types::ToSql::to_sql(jsons, field.type_(), out),
                    "id" => postgres_types::ToSql::to_sql(id, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::max_value() as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "nullity_composite" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 2 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name() {
                        "jsons" => {
                            <&'a [&'a serde_json::value::Value] as postgres_types::ToSql>::accepts(
                                f.type_(),
                            )
                        }
                        "id" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                        _ => false,
                    })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(
        serde::Deserialize,
        serde::Serialize,
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        strum::EnumString,
        strum::AsRefStr,
        std::hash::Hash,
        num_derive::FromPrimitive,
    )]
    #[allow(non_camel_case_types)]
    pub enum SpongebobCharacter {
        Bob,
        Patrick,
        Squidward,
    }
    impl<'a> postgres_types::ToSql for SpongebobCharacter {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            buf: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            buf.extend_from_slice(self.as_ref().as_bytes());
            std::result::Result::Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "spongebob_character" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Enum(ref variants) => {
                    if variants.len() != 3 {
                        return false;
                    }
                    variants.iter().all(|v| Self::from_str(&**v).is_ok())
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    impl<'a> postgres_types::FromSql<'a> for SpongebobCharacter {
        fn from_sql(
            ty: &postgres_types::Type,
            buf: &'a [u8],
        ) -> Result<SpongebobCharacter, Box<dyn std::error::Error + Sync + Send>> {
            let s = std::str::from_utf8(buf)?;
            Ok(Self::from_str(s).map_err(|e| format!("invalid variant `{}`", s))?)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "spongebob_character" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Enum(ref variants) => {
                    if variants.len() != 3 {
                        return false;
                    }
                    variants.iter().all(|v| Self::from_str(&**v).is_ok())
                }
                _ => false,
            }
        }
    }
    #[derive(
        serde::Deserialize, serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq,
    )]
    #[postgres(name = "custom_composite")]
    pub struct CustomComposite {
        #[postgres(name = "wow")]
        pub wow: String,
        #[postgres(name = "such_cool")]
        pub such_cool: i32,
        #[postgres(name = "nice")]
        pub nice: super::public::SpongebobCharacter,
    }
    #[derive(Debug)]
    pub struct CustomCompositeBorrowed<'a> {
        pub wow: &'a str,
        pub such_cool: i32,
        pub nice: super::public::SpongebobCharacter,
    }
    impl<'a> From<CustomCompositeBorrowed<'a>> for CustomComposite {
        fn from(
            CustomCompositeBorrowed {
                wow,
                such_cool,
                nice,
            }: CustomCompositeBorrowed<'a>,
        ) -> Self {
            Self {
                wow: wow.into(),
                such_cool,
                nice,
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for CustomCompositeBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<CustomCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>> {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let wow = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let nice = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(CustomCompositeBorrowed {
                wow,
                such_cool,
                nice,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "custom_composite" && ty.schema() == "public"
        }
    }
    impl<'a> postgres_types::ToSql for CustomCompositeBorrowed<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let CustomCompositeBorrowed {
                wow,
                such_cool,
                nice,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "wow" => postgres_types::ToSql::to_sql(wow, field.type_(), out),
                    "such_cool" => postgres_types::ToSql::to_sql(such_cool, field.type_(), out),
                    "nice" => postgres_types::ToSql::to_sql(nice, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::max_value() as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "custom_composite" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name() {
                        "wow" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                        "such_cool" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                        "nice" => {
                            <super::public::SpongebobCharacter as postgres_types::ToSql>::accepts(
                                f.type_(),
                            )
                        }
                        _ => false,
                    })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(
        serde::Deserialize, serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq,
    )]
    #[postgres(name = "nightmare_composite")]
    pub struct NightmareComposite {
        #[postgres(name = "custom")]
        pub custom: Vec<super::public::CustomComposite>,
        #[postgres(name = "spongebob")]
        pub spongebob: Vec<super::public::SpongebobCharacter>,
        #[postgres(name = "domain")]
        pub domain: String,
    }
    #[derive(Debug)]
    pub struct NightmareCompositeBorrowed<'a> {
        pub custom: cornucopia_async::ArrayIterator<'a, super::public::CustomCompositeBorrowed<'a>>,
        pub spongebob: cornucopia_async::ArrayIterator<'a, super::public::SpongebobCharacter>,
        pub domain: &'a str,
    }
    impl<'a> From<NightmareCompositeBorrowed<'a>> for NightmareComposite {
        fn from(
            NightmareCompositeBorrowed {
                custom,
                spongebob,
                domain,
            }: NightmareCompositeBorrowed<'a>,
        ) -> Self {
            Self {
                custom: custom.map(|v| v.into()).collect(),
                spongebob: spongebob.map(|v| v).collect(),
                domain: domain.into(),
            }
        }
    }
    impl<'a> postgres_types::FromSql<'a> for NightmareCompositeBorrowed<'a> {
        fn from_sql(
            ty: &postgres_types::Type,
            out: &'a [u8],
        ) -> Result<NightmareCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
        {
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            let mut out = out;
            let num_fields = postgres_types::private::read_be_i32(&mut out)?;
            if num_fields as usize != fields.len() {
                return std::result::Result::Err(std::convert::Into::into(format!(
                    "invalid field count: {} vs {}",
                    num_fields,
                    fields.len()
                )));
            }
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let custom = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let spongebob = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
            let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let domain = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
            Ok(NightmareCompositeBorrowed {
                custom,
                spongebob,
                domain,
            })
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            ty.name() == "nightmare_composite" && ty.schema() == "public"
        }
    }
    #[derive(Debug)]
    pub struct NightmareCompositeParams<'a> {
        pub custom: &'a [super::public::CustomCompositeBorrowed<'a>],
        pub spongebob: &'a [super::public::SpongebobCharacter],
        pub domain: &'a str,
    }
    impl<'a> postgres_types::ToSql for NightmareCompositeParams<'a> {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let NightmareCompositeParams {
                custom,
                spongebob,
                domain,
            } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "custom" => postgres_types::ToSql::to_sql(custom, field.type_(), out),
                    "spongebob" => postgres_types::ToSql::to_sql(spongebob, field.type_(), out),
                    "domain" => postgres_types::ToSql::to_sql(
                        &cornucopia_async::private::Domain(domain),
                        field.type_(),
                        out,
                    ),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::max_value() as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "nightmare_composite" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 3 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name()
                {
                    "custom" => <&'a [super::public::CustomCompositeBorrowed<'a>] as
                    postgres_types::ToSql>::accepts(f.type_()),"spongebob" => <&'a [super::public::SpongebobCharacter] as
                    postgres_types::ToSql>::accepts(f.type_()),"domain" => <cornucopia_async::private::Domain::<&'a str> as
                    postgres_types::ToSql>::accepts(f.type_()),_ => false,
                })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(
        serde::Deserialize, serde::Serialize, Debug, postgres_types::FromSql, Copy, Clone, PartialEq,
    )]
    #[postgres(name = "syntax_composite")]
    pub struct SyntaxComposite {
        #[postgres(name = "async")]
        pub r#async: i32,
    }
    impl<'a> postgres_types::ToSql for SyntaxComposite {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            let SyntaxComposite { r#async } = self;
            let fields = match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            };
            out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
            for field in fields {
                out.extend_from_slice(&field.type_().oid().to_be_bytes());
                let base = out.len();
                out.extend_from_slice(&[0; 4]);
                let r = match field.name() {
                    "async" => postgres_types::ToSql::to_sql(r#async, field.type_(), out),
                    _ => unreachable!(),
                };
                let count = match r? {
                    postgres_types::IsNull::Yes => -1,
                    postgres_types::IsNull::No => {
                        let len = out.len() - base - 4;
                        if len > i32::max_value() as usize {
                            return Err(Into::into("value too large to transmit"));
                        }
                        len as i32
                    }
                };
                out[base..base + 4].copy_from_slice(&count.to_be_bytes());
            }
            Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "syntax_composite" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Composite(ref fields) => {
                    if fields.len() != 1 {
                        return false;
                    }
                    fields.iter().all(|f| match f.name() {
                        "async" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                        _ => false,
                    })
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    #[derive(
        serde::Deserialize,
        serde::Serialize,
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        strum::EnumString,
        strum::AsRefStr,
        std::hash::Hash,
        num_derive::FromPrimitive,
    )]
    #[allow(non_camel_case_types)]
    pub enum SyntaxEnum {
        #[strum(serialize = "async")]
        r#async,
        #[strum(serialize = "box")]
        r#box,
        #[strum(serialize = "I Love Chocolate")]
        I_Love_Chocolate,
    }
    impl<'a> postgres_types::ToSql for SyntaxEnum {
        fn to_sql(
            &self,
            ty: &postgres_types::Type,
            buf: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            buf.extend_from_slice(self.as_ref().as_bytes());
            std::result::Result::Ok(postgres_types::IsNull::No)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "syntax_enum" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Enum(ref variants) => {
                    if variants.len() != 3 {
                        return false;
                    }
                    variants.iter().all(|v| Self::from_str(&**v).is_ok())
                }
                _ => false,
            }
        }
        fn to_sql_checked(
            &self,
            ty: &postgres_types::Type,
            out: &mut postgres_types::private::BytesMut,
        ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
            postgres_types::__to_sql_checked(self, ty, out)
        }
    }
    impl<'a> postgres_types::FromSql<'a> for SyntaxEnum {
        fn from_sql(
            ty: &postgres_types::Type,
            buf: &'a [u8],
        ) -> Result<SyntaxEnum, Box<dyn std::error::Error + Sync + Send>> {
            let s = std::str::from_utf8(buf)?;
            Ok(Self::from_str(s).map_err(|e| format!("invalid variant `{}`", s))?)
        }
        fn accepts(ty: &postgres_types::Type) -> bool {
            if ty.name() != "syntax_enum" {
                return false;
            }
            match *ty.kind() {
                postgres_types::Kind::Enum(ref variants) => {
                    if variants.len() != 3 {
                        return false;
                    }
                    variants.iter().all(|v| Self::from_str(&**v).is_ok())
                }
                _ => false,
            }
        }
    }
}
