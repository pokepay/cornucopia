use core::str;
use std::fmt::{Display, Write};

use codegen_template::code;
use indexmap::IndexMap;

use crate::{
    prepare_queries::{
        Ident, Preparation, PreparedContent, PreparedField, PreparedItem, PreparedModule,
        PreparedQuery, PreparedType,
    },
    CodegenSettings,
};

const GENERATED_FILE_HEADER: &str = "// This file was generated with `cornucopia`. Do not modify.";

pub struct GenCtx {
    // Current module depth
    pub depth: u8,
    // Should use async client and generate async code
    pub is_async: bool,
    // Should serializable struct
    pub derive_serde: bool,
    // Should graphql struct
    pub derive_graphql: bool,
}

impl GenCtx {
    pub fn new(depth: u8, is_async: bool, derive_serde: bool, derive_graphql: bool) -> Self {
        Self {
            depth,
            is_async,
            derive_serde,
            derive_graphql,
        }
    }

    pub fn path(&self, depth: u8, name: impl Display) -> String {
        let depth = std::iter::repeat("super::").take(depth as usize);
        code!($($depth)$name)
    }

    pub fn client_name(&self) -> &'static str {
        if self.is_async {
            "cornucopia_async"
        } else {
            "cornucopia_sync"
        }
    }
}

impl PreparedField {
    pub fn own_struct(&self, ctx: &GenCtx) -> String {
        let it = self.ty.own_ty(self.is_inner_nullable, ctx);
        if self.is_nullable {
            format!("Option<{it}>")
        } else {
            it
        }
    }

    pub fn param_ergo_ty(&self, traits: &mut Vec<String>, ctx: &GenCtx) -> String {
        let it = self.ty.param_ergo_ty(self.is_inner_nullable, traits, ctx);
        if self.is_nullable {
            format!("Option<{it}>")
        } else {
            it
        }
    }

    pub fn param_ty(&self, ctx: &GenCtx) -> String {
        let it = self.ty.param_ty(self.is_inner_nullable, ctx);
        if self.is_nullable {
            format!("Option<{it}>")
        } else {
            it
        }
    }

    pub fn brw_ty(&self, has_lifetime: bool, ctx: &GenCtx) -> String {
        let it = self.ty.brw_ty(self.is_inner_nullable, has_lifetime, ctx);
        if self.is_nullable {
            format!("Option<{it}>")
        } else {
            it
        }
    }

    pub fn owning_call(&self, name: Option<&str>) -> String {
        self.ty.owning_call(
            name.unwrap_or(&self.ident.rs),
            self.is_nullable,
            self.is_inner_nullable,
        )
    }

    pub fn owning_assign(&self) -> String {
        let call = self.owning_call(None);
        if call == self.ident.rs {
            call
        } else {
            format!("{}: {call}", self.ident.rs)
        }
    }
}

fn enum_sql(w: &mut impl Write, name: &str, enum_name: &str, variants: &[Ident]) {
    let nb_variants = variants.len();
    code!(w =>
        impl<'a> postgres_types::ToSql for $enum_name {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>,> {
                buf.extend_from_slice(self.as_ref().as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "$name" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != $nb_variants {
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
        impl<'a> postgres_types::FromSql<'a> for $enum_name {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<$enum_name, Box<dyn std::error::Error + Sync + Send>> {
                let s = std::str::from_utf8(buf)?;
                Ok(Self::from_str(s)
                   .map_err(|e| format!("invalid variant `{}`", s))?)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() !=  "$name" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != $nb_variants {
                            return false;
                        }
                        variants.iter().all(|v| Self::from_str(&**v).is_ok())
                    }
                    _ => false,
                }
            }
        }
    );
}

fn struct_tosql(
    w: &mut impl Write,
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    is_borrow: bool,
    is_params: bool,
    ctx: &GenCtx,
) {
    let (post, lifetime) = if is_borrow {
        if is_params {
            ("Borrowed", "<'a>")
        } else {
            ("Params", "<'a>")
        }
    } else {
        ("", "")
    };
    let db_fields_ident = fields.iter().map(|p| &p.ident.db);
    let rs_fields_ident = fields.iter().map(|p| &p.ident.rs);
    let write_ty = fields.iter().map(|p| p.ty.sql_wrapped(&p.ident.rs, ctx));
    let accept_ty = fields.iter().map(|p| p.ty.accept_to_sql(ctx));
    let nb_fields = fields.len();

    code!(w =>
        impl<'a> postgres_types::ToSql for $struct_name$post $lifetime {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>,> {
                let $struct_name$post {
                    $($rs_fields_ident,)
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
                        $("$db_fields_ident" => postgres_types::ToSql::to_sql($write_ty,field.type_(), out),)
                        _ => unreachable!()
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
                if ty.name() != "$name" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != $nb_fields {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            $("$db_fields_ident" => <$accept_ty as postgres_types::ToSql>::accepts(f.type_()),)
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
    );
}

fn composite_fromsql(
    w: &mut impl Write,
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    schema: &str,
) {
    let field_names = fields.iter().map(|p| &p.ident.rs);
    let read_idx = 0..fields.len();
    code!(w =>
        impl<'a> postgres_types::FromSql<'a> for ${struct_name}Borrowed<'a> {
            fn from_sql(ty: &postgres_types::Type, out: &'a [u8]) ->
                Result<${struct_name}Borrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(
                        std::convert::Into::into(format!("invalid field count: {} vs {}", num_fields, fields.len())));
                }
                $(
                    let _oid = postgres_types::private::read_be_i32(&mut out)?;
                    let $field_names = postgres_types::private::read_value(fields[$read_idx].type_(), &mut out)?;
                )
                Ok(${struct_name}Borrowed { $($field_names,) })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "$name" && ty.schema() == "$schema"
            }
        }
    );
}

fn gen_params_struct(w: &mut impl Write, params: &PreparedItem, ctx: &GenCtx) {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
        is_ref,
        ..
    } = params;
    if *is_named {
        let traits = &mut Vec::new();

        let copy = if *is_copy { "Clone,Copy," } else { "" };
        let lifetime = if *is_ref { "'a," } else { "" };
        let fields_ty = fields
            .iter()
            .map(|p| p.param_ergo_ty(traits, ctx))
            .collect::<Vec<_>>();
        let fields_name = fields.iter().map(|p| &p.ident.rs);
        let traits_idx = (1..=traits.len()).map(idx_char);
        code!(w =>
            #[derive($copy Debug)]
            pub struct $name<$lifetime $($traits_idx: $traits,)> {
                $(pub $fields_name: $fields_ty,)
            }
        );
    }
}

fn gen_row_structs(w: &mut impl Write, row: &PreparedItem, ctx: &GenCtx) {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
        ..
    } = row;
    if *is_named {
        // Generate row struct
        let fields_name = fields.iter().map(|p| &p.ident.rs);
        let fields_ty = fields.iter().map(|p| p.own_struct(ctx));
        let copy = if *is_copy { "Copy" } else { "" };
        let derive_serde = if ctx.derive_serde {
            "serde::Deserialize, serde::Serialize,"
        } else {
            ""
        };
        code!(w =>
            #[derive($derive_serde Debug, Clone, PartialEq, $copy)]
            pub struct $name {
                $(pub $fields_name : $fields_ty,)
            }
        );
        if !is_copy {
            let fields_name = fields.iter().map(|p| &p.ident.rs);
            let fields_ty = fields.iter().map(|p| p.brw_ty(true, ctx));
            let from_own_assign = fields.iter().map(|f| f.owning_assign());
            code!(w =>
                pub struct ${name}Borrowed<'a> {
                    $(pub $fields_name : $fields_ty,)
                }
                impl<'a> From<${name}Borrowed<'a>> for $name {
                    fn from(${name}Borrowed { $($fields_name,) }: ${name}Borrowed<'a>) -> Self {
                        Self {
                            $($from_own_assign,)
                        }
                    }
                }
            );
        };
    }
}

fn gen_row_query(w: &mut impl Write, row: &PreparedItem, ctx: &GenCtx) {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
        ..
    } = row;
    // Generate query struct
    let borrowed_str = if *is_copy { "" } else { "Borrowed" };
    let (client_mut, fn_async, fn_await, backend, collect, raw_type, raw_pre, raw_post, client) =
        if ctx.is_async {
            (
                "",
                "async",
                ".await",
                "tokio_postgres",
                "try_collect().await",
                "futures_util::Stream",
                "",
                ".into_stream().boxed()",
                "cornucopia_async",
            )
        } else {
            (
                "mut",
                "",
                "",
                "postgres",
                "collect()",
                "Iterator",
                ".iterator()",
                "",
                "cornucopia_sync",
            )
        };

    let row_struct = if *is_named {
        format!("{}{borrowed_str}", row.path(ctx))
    } else {
        fields[0].brw_ty(false, ctx)
    };

    code!(w =>
    pub struct ${name}Query<'a, C: GenericClient, T, const N: usize> {
        client: &'a $client_mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a $backend::Statement>,
        extractor: fn(&$backend::Row) -> $row_struct,
        mapper: fn($row_struct) -> T,
    }
    impl<'a, C, T:'a, const N: usize> ${name}Query<'a, C, T, N> where C: GenericClient {
        pub fn map<R>(self, mapper: fn($row_struct) -> R) -> ${name}Query<'a,C,R,N> {
            ${name}Query {
                client: self.client,
                params: self.params,
                query: self.query,
                cached: self.cached,
                extractor: self.extractor,
                mapper,
            }
        }

        pub $fn_async fn one(self) -> Result<T, $backend::Error> {
            let row = $client::private::one(self.client, self.query, &self.params, self.cached)$fn_await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }

        pub $fn_async fn all(self) -> Result<Vec<T>, $backend::Error> {
            self.iter()$fn_await?.$collect
        }

        pub $fn_async fn opt(self) -> Result<Option<T>, $backend::Error> {
            let opt_row = $client::private::opt(self.client, self.query, &self.params, self.cached)$fn_await?;
            Ok(opt_row.map(|row| (self.mapper)((self.extractor)(&row))))
        }

        pub $fn_async fn iter(
            self,
        ) -> Result<impl $raw_type<Item = Result<T, $backend::Error>> + 'a, $backend::Error> {
            let stream = $client::private::raw(self.client, self.query, $client::private::slice_iter(&self.params), self.cached)$fn_await?;
            let mapped = stream
                $raw_pre
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                $raw_post;
            Ok(mapped)
        }
    });
}

pub fn idx_char(idx: usize) -> String {
    format!("T{idx}")
}

fn gen_query_fn<W: Write>(w: &mut W, module: &PreparedModule, query: &PreparedQuery, ctx: &GenCtx) {
    let PreparedQuery {
        ident,
        row,
        sql,
        param,
    } = query;

    let (client_mut, fn_async, fn_await, backend, client) = if ctx.is_async {
        ("", "async", ".await", "tokio_postgres", "cornucopia_async")
    } else {
        ("mut", "", "", "postgres", "cornucopia_sync")
    };

    let struct_name = ident.type_ident();
    let (param, param_field, order) = match param {
        Some((idx, order)) => {
            let it = module.params.get_index(*idx).unwrap().1;
            (Some(it), it.fields.as_slice(), order.as_slice())
        }
        None => (None, [].as_slice(), [].as_slice()),
    };
    let traits = &mut Vec::new();
    let params_ty: Vec<_> = order
        .iter()
        .map(|idx| param_field[*idx].param_ergo_ty(traits, ctx))
        .collect();
    let params_name = order.iter().map(|idx| &param_field[*idx].ident.rs);
    let traits_idx = (1..=traits.len()).map(idx_char);
    let lazy_impl = |w: &mut W| {
        if let Some((idx, index)) = row {
            let item = module.rows.get_index(*idx).unwrap().1;
            let PreparedItem {
                name: row_name,
                fields,
                is_copy,
                is_named,
                ..
            } = &item;
            // Query fn
            let nb_params = param_field.len();

            // TODO find a way to clean this mess
            #[allow(clippy::type_complexity)]
            let (row_struct_name, extractor, mapper): (_, Box<dyn Fn(&mut W)>, _) = if *is_named {
                let path = item.path(ctx);
                (
                    path.clone(),
                    Box::new(|w: _| {
                        let path = item.path(ctx);
                        let post = if *is_copy { "" } else { "Borrowed" };
                        let fields_name = fields.iter().map(|p| &p.ident.rs);
                        let fields_idx = (0..fields.len()).map(|i| index[i]);
                        code!(w => $path$post {
                            $($fields_name: row.get($fields_idx),)
                        })
                    }),
                    code!(<$path>::from(it)),
                )
            } else {
                let field = &fields[0];
                (
                    field.own_struct(ctx),
                    Box::new(|w: _| code!(w => row.get(0))),
                    field.owning_call(Some("it")),
                )
            };
            code!(w =>
                pub fn bind<'a, C: GenericClient, $($traits_idx: $traits,)>(&'a self, client: &'a $client_mut C, $($params_name: &'a $params_ty,) ) -> ${row_name}Query<'a,C, $row_struct_name, $nb_params> {
                    ${row_name}Query {
                        client,
                        params: [$($params_name,)],
                        query: self.0,
                        cached: self.1.as_ref(),
                        extractor: |row| { $!extractor },
                        mapper: |it| { $mapper },
                    }
                }
            );
        } else {
            // Execute fn
            let params_wrap = order.iter().map(|idx| {
                let p = &param_field[*idx];
                p.ty.sql_wrapped(&p.ident.rs, ctx)
            });
            code!(w =>
                pub $fn_async fn bind<'a, C: GenericClient, $($traits_idx: $traits,)>(&'a self, client: &'a $client_mut C, $($params_name: &'a $params_ty,)) -> Result<u64, $backend::Error> {
                    client.execute(self.0, &[ $($params_wrap,) ])$fn_await
                }
            );
        }
    };
    // Gen statement struct
    {
        let sql = sql.replace('"', "\\\""); // Rust string format escaping
        let name = &ident.rs;
        code!(w =>
            pub fn $name() -> ${struct_name}Stmt {
                ${struct_name}Stmt("$sql", None)
            }
            pub struct ${struct_name}Stmt(&'static str, Option<$backend::Statement>);
            impl ${struct_name}Stmt {
                pub $fn_async fn prepare<'a, C: GenericClient>(mut self, client: &'a $client_mut C) -> Result<Self, $backend::Error> {
                    self.1 = Some(client.prepare(self.0)$fn_await?);
                    Ok(self)
                }

                $!lazy_impl
            }
        );
    }

    // Param impl
    if let Some(param) = param {
        if param.is_named {
            let param_path = &param.path(ctx);
            let lifetime = if param.is_copy || !param.is_ref {
                ""
            } else {
                "'a,"
            };
            if let Some((idx, _)) = row {
                let prepared_row = &module.rows.get_index(*idx).unwrap().1;
                let query_row_struct = if prepared_row.is_named {
                    prepared_row.path(ctx)
                } else {
                    prepared_row.fields[0].own_struct(ctx)
                };
                let name = &module.rows.get_index(*idx).unwrap().1.name;
                let nb_params = param_field.len();
                code!(w =>
                    impl <'a, C: GenericClient,$($traits_idx: $traits,)> $client::Params<'a, $param_path<$lifetime $($traits_idx,)>, ${name}Query<'a, C, $query_row_struct, $nb_params>, C> for ${struct_name}Stmt {
                        fn params(&'a self, client: &'a $client_mut C, params: &'a $param_path<$lifetime $($traits_idx,)>) -> ${name}Query<'a, C, $query_row_struct, $nb_params> {
                            self.bind(client, $(&params.$params_name,))
                        }
                    }
                );
            } else {
                let (send_sync, pre_ty, post_ty_lf, pre, post) = if ctx.is_async {
                    (
                        "+ Send + Sync",
                        "std::pin::Pin<Box<dyn futures_util::Future<Output = Result",
                        "> + Send + 'a>>",
                        "Box::pin(self",
                        ")",
                    )
                } else {
                    ("", "Result", "", "self", "")
                };
                code!(w =>
                    impl <'a, C: GenericClient $send_sync, $($traits_idx: $traits,)> $client::Params<'a, $param_path<$lifetime $($traits_idx,)>, $pre_ty<u64, $backend::Error>$post_ty_lf, C> for ${struct_name}Stmt {
                        fn params(&'a self, client: &'a $client_mut C, params: &'a $param_path<$lifetime $($traits_idx,)>) -> $pre_ty<u64, $backend::Error>$post_ty_lf {
                            $pre.bind(client, $(&params.$params_name,))$post
                        }
                    }
                );
            }
        }
    }
}

/// Generates type definitions for custom user types. This includes domains, composites and enums.
/// If the type is not `Copy`, then a Borrowed version will be generated.
fn gen_custom_type(w: &mut impl Write, schema: &str, prepared: &PreparedType, ctx: &GenCtx) {
    let PreparedType {
        struct_name,
        content,
        is_copy,
        is_params,
        name,
    } = prepared;
    let copy = if *is_copy { "Copy," } else { "" };
    let derive_serde = if ctx.derive_serde {
        "serde::Deserialize, serde::Serialize,"
    } else {
        ""
    };
    match content {
        PreparedContent::Enum(variants) => {
            let derive_graphql = if ctx.derive_graphql {
                "async_graphql::Enum,"
            } else {
                ""
            };
            let variants_ident = variants.iter().map(|v| {
                if v.db != v.rs {
                    format!(r#"#[strum(serialize = "{}")] {}"#, v.db, v.rs)
                } else {
                    v.rs.to_owned()
                }
            });
            code!(w =>
                #[derive($derive_graphql $derive_serde Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::AsRefStr, std::hash::Hash, num_derive::FromPrimitive)]
                #[allow(non_camel_case_types)]
                pub enum $struct_name {
                    $($variants_ident,)
                }
            );
            enum_sql(w, name, struct_name, variants);
        }
        PreparedContent::Composite(fields) => {
            let fields_original_name = fields.iter().map(|p| &p.ident.db);
            let fields_name = fields.iter().map(|p| &p.ident.rs);
            {
                let fields_ty = fields.iter().map(|p| p.own_struct(ctx));
                code!(w =>
                    #[derive($derive_serde Debug,postgres_types::FromSql,$copy Clone, PartialEq)]
                    #[postgres(name = "$name")]
                    pub struct $struct_name {
                        $(
                            #[postgres(name = "$fields_original_name")]
                            pub $fields_name: $fields_ty,
                        )
                    }
                );
            }
            if *is_copy {
                struct_tosql(w, struct_name, fields, name, false, *is_params, ctx);
            } else {
                let fields_owning = fields.iter().map(|p| p.owning_assign());
                let fields_brw = fields.iter().map(|p| p.brw_ty(true, ctx));
                code!(w =>
                    #[derive(Debug)]
                    pub struct ${struct_name}Borrowed<'a> {
                        $(pub $fields_name: $fields_brw,)
                    }
                    impl<'a> From<${struct_name}Borrowed<'a>> for $struct_name {
                        fn from(
                            ${struct_name}Borrowed {
                            $($fields_name,)
                            }: ${struct_name}Borrowed<'a>,
                        ) -> Self {
                            Self {
                                $($fields_owning,)
                            }
                        }
                    }
                );
                composite_fromsql(w, struct_name, fields, name, schema);
                if !is_params {
                    let fields_ty = fields.iter().map(|p| p.param_ty(ctx));
                    let derive = if *is_copy { ",Copy,Clone" } else { "" };
                    code!(w =>
                        #[derive(Debug $derive)]
                        pub struct ${struct_name}Params<'a> {
                            $(pub $fields_name: $fields_ty,)
                        }
                    );
                }
                struct_tosql(w, struct_name, fields, name, true, *is_params, ctx);
            }
        }
    }
}

fn gen_type_modules(
    parent_body_w: &mut String,
    tree: &mut GeneratedFileTree,
    prepared: &IndexMap<String, Vec<PreparedType>>,
    ctx: &GenCtx,
) {
    if prepared.is_empty() {
        return;
    }

    parent_body_w.push_str("pub mod types;\n");

    let types_file = "types.rs".to_owned();
    let mut types_body = format!("{GENERATED_FILE_HEADER}\n\n");
    let w = &mut types_body;

    let modules = prepared.iter().map(|(schema, types)| {
        move |w: &mut String| {
            let lazy = |w: &mut String| {
                for ty in types {
                    gen_custom_type(w, schema, ty, ctx)
                }
            };

            code!(w =>
            pub mod $schema {
                use std::str::FromStr;
                $!lazy
            });
        }
    });

    code!(w =>
          #![allow(clippy::all, clippy::pedantic)]
          #![allow(unused_variables)]
          #![allow(unused_imports)]
          #![allow(dead_code)]


          $($!modules)
    );

    tree.push((types_file, types_body));
}

fn gen_query_modules(
    parent_body_w: &mut String,
    tree: &mut GeneratedFileTree,
    modules: &[PreparedModule],
    settings: &CodegenSettings,
) {
    if modules.is_empty() {
        return;
    }

    parent_body_w.push_str("pub mod queries;\n");

    let query_root_file = "queries/mod.rs".to_owned();
    let mut query_root_body = format!("{GENERATED_FILE_HEADER}\n\n");
    let query_root_body_w = &mut query_root_body;

    // Generate queries
    for module in modules.iter() {
        let name = &module.info.name;
        let ctx = GenCtx::new(
            2,
            settings.gen_async,
            settings.derive_serde,
            settings.derive_graphql,
        );
        let params_string = module
            .params
            .values()
            .map(|params| |w: &mut String| gen_params_struct(w, params, &ctx));
        let rows_struct_string = module
            .rows
            .values()
            .map(|row| |w: &mut String| gen_row_structs(w, row, &ctx));
        let sync_specific = |w: &mut String| {
            let gen_specific = |depth: u8, is_async: bool| {
                move |w: &mut String| {
                    let ctx = GenCtx::new(
                        depth,
                        is_async,
                        settings.derive_serde,
                        settings.derive_graphql,
                    );
                    let import = if is_async {
                        "use futures_util::{StreamExt, TryStreamExt}; use cornucopia_async::GenericClient;"
                    } else {
                        "use postgres::{fallible_iterator::FallibleIterator}; use cornucopia_sync::GenericClient;"
                    };
                    let rows_query_string = module
                        .rows
                        .values()
                        .map(|row| |w: &mut String| gen_row_query(w, row, &ctx));
                    let queries_string = module
                        .queries
                        .values()
                        .map(|query| |w: &mut String| gen_query_fn(w, module, query, &ctx));
                    code!(w =>
                        $import
                        $($!rows_query_string)
                        $($!queries_string)
                    )
                }
            };
            if settings.gen_async != settings.gen_sync {
                if settings.gen_async {
                    let gen = gen_specific(2, true);
                    code!(w => $!gen)
                } else {
                    let gen = gen_specific(2, false);
                    code!(w => $!gen)
                }
            } else {
                let sync = gen_specific(3, false);
                let async_ = gen_specific(3, true);
                code!(w =>
                    pub mod sync {
                        $!sync
                    }
                    pub mod async_ {
                        $!async_
                    }
                )
            }
        };

        query_root_body_w
            .write_fmt(format_args!("pub mod {};\n", name))
            .expect("write_fmt");

        let query_file = format!("queries/{}.rs", name);
        let mut query_body = format!("{GENERATED_FILE_HEADER}\n\n");
        let w = &mut query_body;

        code!(w =>
              #![allow(clippy::all, clippy::pedantic)]
              #![allow(unused_variables)]
              #![allow(unused_imports)]
              #![allow(dead_code)]


              $($!params_string)
              $($!rows_struct_string)
              $!sync_specific
        );

        tree.push((query_file, query_body));
    }

    tree.push((query_root_file, query_root_body));
}

pub type GeneratedFileTree = Vec<(String, String)>;

pub(crate) fn generate(preparation: Preparation, settings: CodegenSettings) -> GeneratedFileTree {
    let mut tree: GeneratedFileTree = Vec::new();

    let root_file = "mod.rs".to_owned();
    let mut root_body = format!("{GENERATED_FILE_HEADER}\n\n");

    // Generate database type
    gen_type_modules(
        &mut root_body,
        &mut tree,
        &preparation.types,
        &GenCtx::new(
            1,
            settings.gen_async,
            settings.derive_serde,
            settings.derive_graphql,
        ),
    );

    // Generate database query
    gen_query_modules(&mut root_body, &mut tree, &preparation.modules, &settings);

    tree.push((root_file, root_body));

    tree
}
