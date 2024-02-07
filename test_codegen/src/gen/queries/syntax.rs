// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#[derive(Debug)]
pub struct ImplicitCompactParams<T1: cornucopia_async::StringSql> {
    pub name: Option<T1>,
    pub price: Option<f64>,
}
#[derive(Debug)]
pub struct ImplicitSpacedParams<T1: cornucopia_async::StringSql> {
    pub name: Option<T1>,
    pub price: Option<f64>,
}
#[derive(Debug)]
pub struct Params<T1: cornucopia_async::StringSql> {
    pub name: T1,
    pub price: f64,
}
#[derive(Debug)]
pub struct ParamsSpace<T1: cornucopia_async::StringSql> {
    pub name: T1,
    pub price: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySqlParams {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql1Params {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql2Params {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql3Params {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql4Params {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql6Params {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql7Params {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql8Params {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql9Params {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(Clone, Copy, Debug)]
pub struct TrickySql10Params {
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Copy)]
pub struct Row {
    pub id: i32,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Copy)]
pub struct RowSpace {
    pub id: i32,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct Typeof {
    pub trick_y: String,
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
pub struct TypeofBorrowed<'a> {
    pub trick_y: &'a str,
    pub r#async: super::super::types::public::SyntaxComposite,
    pub r#enum: super::super::types::public::SyntaxEnum,
}
impl<'a> From<TypeofBorrowed<'a>> for Typeof {
    fn from(
        TypeofBorrowed {
            trick_y,
            r#async,
            r#enum,
        }: TypeofBorrowed<'a>,
    ) -> Self {
        Self {
            trick_y: trick_y.into(),
            r#async,
            r#enum,
        }
    }
}
pub mod sync {
    use cornucopia_sync::GenericClient;
    use postgres::fallible_iterator::FallibleIterator;
    pub struct PublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::super::super::types::public::CloneCompositeBorrowed,
        mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> PublicCloneCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> R,
        ) -> PublicCloneCompositeQuery<'a, C, R, N> {
            PublicCloneCompositeQuery {
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
    pub struct Optioni32Query<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> Option<i32>,
        mapper: fn(Option<i32>) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> Optioni32Query<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'a, C, R, N> {
            Optioni32Query {
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
    pub struct RowQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::Row,
        mapper: fn(super::Row) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> RowQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'a, C, R, N> {
            RowQuery {
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
    pub struct RowSpaceQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::RowSpace,
        mapper: fn(super::RowSpace) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> RowSpaceQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::RowSpace) -> R) -> RowSpaceQuery<'a, C, R, N> {
            RowSpaceQuery {
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
    pub struct TypeofQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a postgres::Statement>,
        extractor: fn(&postgres::Row) -> super::TypeofBorrowed,
        mapper: fn(super::TypeofBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> TypeofQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::TypeofBorrowed) -> R) -> TypeofQuery<'a, C, R, N> {
            TypeofQuery {
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
    pub fn select_compact() -> SelectCompactStmt {
        SelectCompactStmt("SELECT * FROM clone", None)
    }
    pub struct SelectCompactStmt(&'static str, Option<postgres::Statement>);
    impl SelectCompactStmt {
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
        ) -> PublicCloneCompositeQuery<'a, C, super::super::super::types::public::CloneComposite, 0>
        {
            PublicCloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn select_spaced() -> SelectSpacedStmt {
        SelectSpacedStmt("      SELECT * FROM clone ", None)
    }
    pub struct SelectSpacedStmt(&'static str, Option<postgres::Statement>);
    impl SelectSpacedStmt {
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
        ) -> PublicCloneCompositeQuery<'a, C, super::super::super::types::public::CloneComposite, 0>
        {
            PublicCloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn implicit_compact() -> ImplicitCompactStmt {
        ImplicitCompactStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct ImplicitCompactStmt(&'static str, Option<postgres::Statement>);
    impl ImplicitCompactStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
            &'a self,
            client: &'a mut C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
        cornucopia_sync::Params<
            'a,
            super::ImplicitCompactParams<T1>,
            Optioni32Query<'a, C, Option<i32>, 2>,
            C,
        > for ImplicitCompactStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::ImplicitCompactParams<T1>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn implicit_spaced() -> ImplicitSpacedStmt {
        ImplicitSpacedStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct ImplicitSpacedStmt(&'static str, Option<postgres::Statement>);
    impl ImplicitSpacedStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
            &'a self,
            client: &'a mut C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
        cornucopia_sync::Params<
            'a,
            super::ImplicitSpacedParams<T1>,
            Optioni32Query<'a, C, Option<i32>, 2>,
            C,
        > for ImplicitSpacedStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::ImplicitSpacedParams<T1>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_compact() -> NamedCompactStmt {
        NamedCompactStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct NamedCompactStmt(&'static str, Option<postgres::Statement>);
    impl NamedCompactStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
            &'a self,
            client: &'a mut C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowQuery<'a, C, super::Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::Row { id: row.get(0) },
                mapper: |it| <super::Row>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
        cornucopia_sync::Params<'a, super::Params<T1>, RowQuery<'a, C, super::Row, 2>, C>
        for NamedCompactStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::Params<T1>,
        ) -> RowQuery<'a, C, super::Row, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_spaced() -> NamedSpacedStmt {
        NamedSpacedStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct NamedSpacedStmt(&'static str, Option<postgres::Statement>);
    impl NamedSpacedStmt {
        pub fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a mut C,
        ) -> Result<Self, postgres::Error> {
            self.1 = Some(client.prepare(self.0)?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
            &'a self,
            client: &'a mut C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
            RowSpaceQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::RowSpace { id: row.get(0) },
                mapper: |it| <super::RowSpace>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
        cornucopia_sync::Params<
            'a,
            super::ParamsSpace<T1>,
            RowSpaceQuery<'a, C, super::RowSpace, 2>,
            C,
        > for NamedSpacedStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::ParamsSpace<T1>,
        ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn tricky_sql() -> TrickySqlStmt {
        TrickySqlStmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\', $1, $2)", None)
    }
    pub struct TrickySqlStmt(&'static str, Option<postgres::Statement>);
    impl TrickySqlStmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySqlParams, Result<u64, postgres::Error>, C>
        for TrickySqlStmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySqlParams,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql1() -> TrickySql1Stmt {
        TrickySql1Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)", None)
    }
    pub struct TrickySql1Stmt(&'static str, Option<postgres::Statement>);
    impl TrickySql1Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySql1Params, Result<u64, postgres::Error>, C>
        for TrickySql1Stmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySql1Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql2() -> TrickySql2Stmt {
        TrickySql2Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)", None)
    }
    pub struct TrickySql2Stmt(&'static str, Option<postgres::Statement>);
    impl TrickySql2Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySql2Params, Result<u64, postgres::Error>, C>
        for TrickySql2Stmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySql2Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql3() -> TrickySql3Stmt {
        TrickySql3Stmt("INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)", None)
    }
    pub struct TrickySql3Stmt(&'static str, Option<postgres::Statement>);
    impl TrickySql3Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySql3Params, Result<u64, postgres::Error>, C>
        for TrickySql3Stmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySql3Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql4() -> TrickySql4Stmt {
        TrickySql4Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)", None)
    }
    pub struct TrickySql4Stmt(&'static str, Option<postgres::Statement>);
    impl TrickySql4Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySql4Params, Result<u64, postgres::Error>, C>
        for TrickySql4Stmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySql4Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql6() -> TrickySql6Stmt {
        TrickySql6Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)", None)
    }
    pub struct TrickySql6Stmt(&'static str, Option<postgres::Statement>);
    impl TrickySql6Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySql6Params, Result<u64, postgres::Error>, C>
        for TrickySql6Stmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySql6Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql7() -> TrickySql7Stmt {
        TrickySql7Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \':bind_param\'', $1, $2)", None)
    }
    pub struct TrickySql7Stmt(&'static str, Option<postgres::Statement>);
    impl TrickySql7Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySql7Params, Result<u64, postgres::Error>, C>
        for TrickySql7Stmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySql7Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql8() -> TrickySql8Stmt {
        TrickySql8Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \':bind_param\'', $1, $2)", None)
    }
    pub struct TrickySql8Stmt(&'static str, Option<postgres::Statement>);
    impl TrickySql8Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySql8Params, Result<u64, postgres::Error>, C>
        for TrickySql8Stmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySql8Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql9() -> TrickySql9Stmt {
        TrickySql9Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \'not\' a \':bind_param\'', $1, $2)", None)
    }
    pub struct TrickySql9Stmt(&'static str, Option<postgres::Statement>);
    impl TrickySql9Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySql9Params, Result<u64, postgres::Error>, C>
        for TrickySql9Stmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySql9Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn tricky_sql10() -> TrickySql10Stmt {
        TrickySql10Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)", None)
    }
    pub struct TrickySql10Stmt(&'static str, Option<postgres::Statement>);
    impl TrickySql10Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, postgres::Error> {
            client.execute(self.0, &[r#async, r#enum])
        }
    }
    impl<'a, C: GenericClient>
        cornucopia_sync::Params<'a, super::TrickySql10Params, Result<u64, postgres::Error>, C>
        for TrickySql10Stmt
    {
        fn params(
            &'a self,
            client: &'a mut C,
            params: &'a super::TrickySql10Params,
        ) -> Result<u64, postgres::Error> {
            self.bind(client, &params.r#async, &params.r#enum)
        }
    }
    pub fn r#typeof() -> RTypeofStmt {
        RTypeofStmt("SELECT * FROM syntax", None)
    }
    pub struct RTypeofStmt(&'static str, Option<postgres::Statement>);
    impl RTypeofStmt {
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
        ) -> TypeofQuery<'a, C, super::Typeof, 0> {
            TypeofQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::TypeofBorrowed {
                    trick_y: row.get(0),
                    r#async: row.get(1),
                    r#enum: row.get(2),
                },
                mapper: |it| <super::Typeof>::from(it),
            }
        }
    }
}
pub mod async_ {
    use cornucopia_async::GenericClient;
    use futures_util::{StreamExt, TryStreamExt};
    pub struct PublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor:
            fn(&tokio_postgres::Row) -> super::super::super::types::public::CloneCompositeBorrowed,
        mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> PublicCloneCompositeQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(
            self,
            mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> R,
        ) -> PublicCloneCompositeQuery<'a, C, R, N> {
            PublicCloneCompositeQuery {
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
    pub struct Optioni32Query<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> Option<i32>,
        mapper: fn(Option<i32>) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> Optioni32Query<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'a, C, R, N> {
            Optioni32Query {
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
    pub struct RowQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::Row,
        mapper: fn(super::Row) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> RowQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'a, C, R, N> {
            RowQuery {
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
    pub struct RowSpaceQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::RowSpace,
        mapper: fn(super::RowSpace) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> RowSpaceQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::RowSpace) -> R) -> RowSpaceQuery<'a, C, R, N> {
            RowSpaceQuery {
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
    pub struct TypeofQuery<'a, C: GenericClient, T, const N: usize> {
        client: &'a C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        query: &'static str,
        cached: Option<&'a tokio_postgres::Statement>,
        extractor: fn(&tokio_postgres::Row) -> super::TypeofBorrowed,
        mapper: fn(super::TypeofBorrowed) -> T,
    }
    impl<'a, C, T: 'a, const N: usize> TypeofQuery<'a, C, T, N>
    where
        C: GenericClient,
    {
        pub fn map<R>(self, mapper: fn(super::TypeofBorrowed) -> R) -> TypeofQuery<'a, C, R, N> {
            TypeofQuery {
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
    pub fn select_compact() -> SelectCompactStmt {
        SelectCompactStmt("SELECT * FROM clone", None)
    }
    pub struct SelectCompactStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectCompactStmt {
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
        ) -> PublicCloneCompositeQuery<'a, C, super::super::super::types::public::CloneComposite, 0>
        {
            PublicCloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn select_spaced() -> SelectSpacedStmt {
        SelectSpacedStmt("      SELECT * FROM clone ", None)
    }
    pub struct SelectSpacedStmt(&'static str, Option<tokio_postgres::Statement>);
    impl SelectSpacedStmt {
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
        ) -> PublicCloneCompositeQuery<'a, C, super::super::super::types::public::CloneComposite, 0>
        {
            PublicCloneCompositeQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it.into(),
            }
        }
    }
    pub fn implicit_compact() -> ImplicitCompactStmt {
        ImplicitCompactStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct ImplicitCompactStmt(&'static str, Option<tokio_postgres::Statement>);
    impl ImplicitCompactStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
            &'a self,
            client: &'a C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
        cornucopia_async::Params<
            'a,
            super::ImplicitCompactParams<T1>,
            Optioni32Query<'a, C, Option<i32>, 2>,
            C,
        > for ImplicitCompactStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::ImplicitCompactParams<T1>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn implicit_spaced() -> ImplicitSpacedStmt {
        ImplicitSpacedStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct ImplicitSpacedStmt(&'static str, Option<tokio_postgres::Statement>);
    impl ImplicitSpacedStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
            &'a self,
            client: &'a C,
            name: &'a Option<T1>,
            price: &'a Option<f64>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            Optioni32Query {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| row.get(0),
                mapper: |it| it,
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
        cornucopia_async::Params<
            'a,
            super::ImplicitSpacedParams<T1>,
            Optioni32Query<'a, C, Option<i32>, 2>,
            C,
        > for ImplicitSpacedStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::ImplicitSpacedParams<T1>,
        ) -> Optioni32Query<'a, C, Option<i32>, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_compact() -> NamedCompactStmt {
        NamedCompactStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct NamedCompactStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NamedCompactStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
            &'a self,
            client: &'a C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowQuery<'a, C, super::Row, 2> {
            RowQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::Row { id: row.get(0) },
                mapper: |it| <super::Row>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
        cornucopia_async::Params<'a, super::Params<T1>, RowQuery<'a, C, super::Row, 2>, C>
        for NamedCompactStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::Params<T1>,
        ) -> RowQuery<'a, C, super::Row, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn named_spaced() -> NamedSpacedStmt {
        NamedSpacedStmt(
            "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
            None,
        )
    }
    pub struct NamedSpacedStmt(&'static str, Option<tokio_postgres::Statement>);
    impl NamedSpacedStmt {
        pub async fn prepare<'a, C: GenericClient>(
            mut self,
            client: &'a C,
        ) -> Result<Self, tokio_postgres::Error> {
            self.1 = Some(client.prepare(self.0).await?);
            Ok(self)
        }
        pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
            &'a self,
            client: &'a C,
            name: &'a T1,
            price: &'a f64,
        ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
            RowSpaceQuery {
                client,
                params: [name, price],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::RowSpace { id: row.get(0) },
                mapper: |it| <super::RowSpace>::from(it),
            }
        }
    }
    impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
        cornucopia_async::Params<
            'a,
            super::ParamsSpace<T1>,
            RowSpaceQuery<'a, C, super::RowSpace, 2>,
            C,
        > for NamedSpacedStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::ParamsSpace<T1>,
        ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
            self.bind(client, &params.name, &params.price)
        }
    }
    pub fn tricky_sql() -> TrickySqlStmt {
        TrickySqlStmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\', $1, $2)", None)
    }
    pub struct TrickySqlStmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySqlStmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySqlParams,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySqlStmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySqlParams,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql1() -> TrickySql1Stmt {
        TrickySql1Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)", None)
    }
    pub struct TrickySql1Stmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySql1Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySql1Params,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySql1Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql1Params,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql2() -> TrickySql2Stmt {
        TrickySql2Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)", None)
    }
    pub struct TrickySql2Stmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySql2Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySql2Params,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySql2Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql2Params,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql3() -> TrickySql3Stmt {
        TrickySql3Stmt("INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)", None)
    }
    pub struct TrickySql3Stmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySql3Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySql3Params,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySql3Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql3Params,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql4() -> TrickySql4Stmt {
        TrickySql4Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)", None)
    }
    pub struct TrickySql4Stmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySql4Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySql4Params,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySql4Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql4Params,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql6() -> TrickySql6Stmt {
        TrickySql6Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)", None)
    }
    pub struct TrickySql6Stmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySql6Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySql6Params,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySql6Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql6Params,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql7() -> TrickySql7Stmt {
        TrickySql7Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \':bind_param\'', $1, $2)", None)
    }
    pub struct TrickySql7Stmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySql7Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySql7Params,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySql7Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql7Params,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql8() -> TrickySql8Stmt {
        TrickySql8Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \':bind_param\'', $1, $2)", None)
    }
    pub struct TrickySql8Stmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySql8Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySql8Params,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySql8Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql8Params,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql9() -> TrickySql9Stmt {
        TrickySql9Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \'not\' a \':bind_param\'', $1, $2)", None)
    }
    pub struct TrickySql9Stmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySql9Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySql9Params,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySql9Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql9Params,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn tricky_sql10() -> TrickySql10Stmt {
        TrickySql10Stmt("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)", None)
    }
    pub struct TrickySql10Stmt(&'static str, Option<tokio_postgres::Statement>);
    impl TrickySql10Stmt {
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
            r#async: &'a super::super::super::types::public::SyntaxComposite,
            r#enum: &'a super::super::super::types::public::SyntaxEnum,
        ) -> Result<u64, tokio_postgres::Error> {
            client.execute(self.0, &[r#async, r#enum]).await
        }
    }
    impl<'a, C: GenericClient + Send + Sync>
        cornucopia_async::Params<
            'a,
            super::TrickySql10Params,
            std::pin::Pin<
                Box<
                    dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>>
                        + Send
                        + 'a,
                >,
            >,
            C,
        > for TrickySql10Stmt
    {
        fn params(
            &'a self,
            client: &'a C,
            params: &'a super::TrickySql10Params,
        ) -> std::pin::Pin<
            Box<dyn futures_util::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        > {
            Box::pin(self.bind(client, &params.r#async, &params.r#enum))
        }
    }
    pub fn r#typeof() -> RTypeofStmt {
        RTypeofStmt("SELECT * FROM syntax", None)
    }
    pub struct RTypeofStmt(&'static str, Option<tokio_postgres::Statement>);
    impl RTypeofStmt {
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
        ) -> TypeofQuery<'a, C, super::Typeof, 0> {
            TypeofQuery {
                client,
                params: [],
                query: self.0,
                cached: self.1.as_ref(),
                extractor: |row| super::TypeofBorrowed {
                    trick_y: row.get(0),
                    r#async: row.get(1),
                    r#enum: row.get(2),
                },
                mapper: |it| <super::Typeof>::from(it),
            }
        }
    }
}
