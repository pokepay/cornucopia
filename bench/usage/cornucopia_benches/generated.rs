// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod bench {
        #[derive(Debug)]
        pub struct InsertUserParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub name: T1,
            pub hair_color: Option<T2>,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct User {
            pub id: i32,
            pub name: String,
            pub hair_color: Option<String>,
        }
        pub struct UserBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub hair_color: Option<&'a str>,
        }
        impl<'a> From<UserBorrowed<'a>> for User {
            fn from(
                UserBorrowed {
                    id,
                    name,
                    hair_color,
                }: UserBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    name: name.into(),
                    hair_color: hair_color.map(|v| v.into()),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Post {
            pub id: i32,
            pub user_id: i32,
            pub title: String,
            pub body: Option<String>,
        }
        pub struct PostBorrowed<'a> {
            pub id: i32,
            pub user_id: i32,
            pub title: &'a str,
            pub body: Option<&'a str>,
        }
        impl<'a> From<PostBorrowed<'a>> for Post {
            fn from(
                PostBorrowed {
                    id,
                    user_id,
                    title,
                    body,
                }: PostBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    user_id,
                    title: title.into(),
                    body: body.map(|v| v.into()),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Comment {
            pub id: i32,
            pub post_id: i32,
            pub text: String,
        }
        pub struct CommentBorrowed<'a> {
            pub id: i32,
            pub post_id: i32,
            pub text: &'a str,
        }
        impl<'a> From<CommentBorrowed<'a>> for Comment {
            fn from(CommentBorrowed { id, post_id, text }: CommentBorrowed<'a>) -> Self {
                Self {
                    id,
                    post_id,
                    text: text.into(),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectComplex {
            pub myuser_id: i32,
            pub name: String,
            pub hair_color: Option<String>,
            pub post_id: Option<i32>,
            pub user_id: Option<i32>,
            pub title: Option<String>,
            pub body: Option<String>,
        }
        pub struct SelectComplexBorrowed<'a> {
            pub myuser_id: i32,
            pub name: &'a str,
            pub hair_color: Option<&'a str>,
            pub post_id: Option<i32>,
            pub user_id: Option<i32>,
            pub title: Option<&'a str>,
            pub body: Option<&'a str>,
        }
        impl<'a> From<SelectComplexBorrowed<'a>> for SelectComplex {
            fn from(
                SelectComplexBorrowed {
                    myuser_id,
                    name,
                    hair_color,
                    post_id,
                    user_id,
                    title,
                    body,
                }: SelectComplexBorrowed<'a>,
            ) -> Self {
                Self {
                    myuser_id,
                    name: name.into(),
                    hair_color: hair_color.map(|v| v.into()),
                    post_id,
                    user_id,
                    title: title.map(|v| v.into()),
                    body: body.map(|v| v.into()),
                }
            }
        }
        pub mod sync {
            use postgres::{fallible_iterator::FallibleIterator, GenericClient};
            pub struct UserQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                query: &'static str,
                extractor: fn(&postgres::Row) -> super::UserBorrowed,
                mapper: fn(super::UserBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> UserQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::UserBorrowed) -> R,
                ) -> UserQuery<'a, C, R, N> {
                    UserQuery {
                        client: self.client,
                        params: self.params,
                        query: self.query,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.client.prepare(self.query)?;
                    let row = self.client.query_one(&stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.client.prepare(self.query)?;
                    Ok(self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.client.prepare(self.query)?;
                    let it = self
                        .client
                        .query_raw(&stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct PostQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                query: &'static str,
                extractor: fn(&postgres::Row) -> super::PostBorrowed,
                mapper: fn(super::PostBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PostQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::PostBorrowed) -> R,
                ) -> PostQuery<'a, C, R, N> {
                    PostQuery {
                        client: self.client,
                        params: self.params,
                        query: self.query,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.client.prepare(self.query)?;
                    let row = self.client.query_one(&stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.client.prepare(self.query)?;
                    Ok(self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.client.prepare(self.query)?;
                    let it = self
                        .client
                        .query_raw(&stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct CommentQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                query: &'static str,
                extractor: fn(&postgres::Row) -> super::CommentBorrowed,
                mapper: fn(super::CommentBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> CommentQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::CommentBorrowed) -> R,
                ) -> CommentQuery<'a, C, R, N> {
                    CommentQuery {
                        client: self.client,
                        params: self.params,
                        query: self.query,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.client.prepare(self.query)?;
                    let row = self.client.query_one(&stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.client.prepare(self.query)?;
                    Ok(self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.client.prepare(self.query)?;
                    let it = self
                        .client
                        .query_raw(&stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct SelectComplexQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                query: &'static str,
                extractor: fn(&postgres::Row) -> super::SelectComplexBorrowed,
                mapper: fn(super::SelectComplexBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SelectComplexQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::SelectComplexBorrowed) -> R,
                ) -> SelectComplexQuery<'a, C, R, N> {
                    SelectComplexQuery {
                        client: self.client,
                        params: self.params,
                        query: self.query,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.client.prepare(self.query)?;
                    let row = self.client.query_one(&stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.client.prepare(self.query)?;
                    Ok(self
                        .client
                        .query_opt(&stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.client.prepare(self.query)?;
                    let it = self
                        .client
                        .query_raw(&stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub fn users() -> UsersStmt {
                UsersStmt("SELECT * FROM users")
            }
            pub struct UsersStmt(&'static str);
            impl UsersStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a self,
                    client: &'a mut C,
                ) -> UserQuery<'a, C, super::User, 0> {
                    UserQuery {
                        client,
                        params: [],
                        query: self.0,
                        extractor: |row| super::UserBorrowed {
                            id: row.get(0),
                            name: row.get(1),
                            hair_color: row.get(2),
                        },
                        mapper: |it| <super::User>::from(it),
                    }
                }
            }
            pub fn insert_user() -> InsertUserStmt {
                InsertUserStmt("INSERT INTO users (name, hair_color) VALUES ($1, $2)")
            }
            pub struct InsertUserStmt(&'static str);
            impl InsertUserStmt {
                pub fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::StringSql,
                >(
                    &'a self,
                    client: &'a mut C,
                    name: &'a T1,
                    hair_color: &'a Option<T2>,
                ) -> Result<u64, postgres::Error> {
                    let stmt = client.prepare(self.0)?;
                    client.execute(&stmt, &[name, hair_color])
                }
            }
            impl<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::StringSql,
                >
                cornucopia_sync::Params<
                    'a,
                    super::InsertUserParams<T1, T2>,
                    Result<u64, postgres::Error>,
                    C,
                > for InsertUserStmt
            {
                fn params(
                    &'a self,
                    client: &'a mut C,
                    params: &'a super::InsertUserParams<T1, T2>,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.name, &params.hair_color)
                }
            }
            pub fn posts() -> PostsStmt {
                PostsStmt("SELECT * FROM posts")
            }
            pub struct PostsStmt(&'static str);
            impl PostsStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a self,
                    client: &'a mut C,
                ) -> PostQuery<'a, C, super::Post, 0> {
                    PostQuery {
                        client,
                        params: [],
                        query: self.0,
                        extractor: |row| super::PostBorrowed {
                            id: row.get(0),
                            user_id: row.get(1),
                            title: row.get(2),
                            body: row.get(3),
                        },
                        mapper: |it| <super::Post>::from(it),
                    }
                }
            }
            pub fn post_by_user_ids() -> PostByUserIdsStmt {
                PostByUserIdsStmt("SELECT * FROM posts WHERE user_id = ANY($1)")
            }
            pub struct PostByUserIdsStmt(&'static str);
            impl PostByUserIdsStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::ArraySql<Item = i32>>(
                    &'a self,
                    client: &'a mut C,
                    ids: &'a T1,
                ) -> PostQuery<'a, C, super::Post, 1> {
                    PostQuery {
                        client,
                        params: [ids],
                        query: self.0,
                        extractor: |row| super::PostBorrowed {
                            id: row.get(0),
                            user_id: row.get(1),
                            title: row.get(2),
                            body: row.get(3),
                        },
                        mapper: |it| <super::Post>::from(it),
                    }
                }
            }
            pub fn comments() -> CommentsStmt {
                CommentsStmt("SELECT * FROM comments")
            }
            pub struct CommentsStmt(&'static str);
            impl CommentsStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a self,
                    client: &'a mut C,
                ) -> CommentQuery<'a, C, super::Comment, 0> {
                    CommentQuery {
                        client,
                        params: [],
                        query: self.0,
                        extractor: |row| super::CommentBorrowed {
                            id: row.get(0),
                            post_id: row.get(1),
                            text: row.get(2),
                        },
                        mapper: |it| <super::Comment>::from(it),
                    }
                }
            }
            pub fn comments_by_post_id() -> CommentsByPostIdStmt {
                CommentsByPostIdStmt("SELECT * FROM comments WHERE post_id = ANY($1)")
            }
            pub struct CommentsByPostIdStmt(&'static str);
            impl CommentsByPostIdStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::ArraySql<Item = i32>>(
                    &'a self,
                    client: &'a mut C,
                    ids: &'a T1,
                ) -> CommentQuery<'a, C, super::Comment, 1> {
                    CommentQuery {
                        client,
                        params: [ids],
                        query: self.0,
                        extractor: |row| super::CommentBorrowed {
                            id: row.get(0),
                            post_id: row.get(1),
                            text: row.get(2),
                        },
                        mapper: |it| <super::Comment>::from(it),
                    }
                }
            }
            pub fn select_complex() -> SelectComplexStmt {
                SelectComplexStmt("SELECT u.id as myuser_id, u.name, u.hair_color, p.id as post_id, p.user_id, p.title, p.body FROM users as u LEFT JOIN posts as p on u.id = p.user_id")
            }
            pub struct SelectComplexStmt(&'static str);
            impl SelectComplexStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a self,
                    client: &'a mut C,
                ) -> SelectComplexQuery<'a, C, super::SelectComplex, 0> {
                    SelectComplexQuery {
                        client,
                        params: [],
                        query: self.0,
                        extractor: |row| super::SelectComplexBorrowed {
                            myuser_id: row.get(0),
                            name: row.get(1),
                            hair_color: row.get(2),
                            post_id: row.get(3),
                            user_id: row.get(4),
                            title: row.get(5),
                            body: row.get(6),
                        },
                        mapper: |it| <super::SelectComplex>::from(it),
                    }
                }
            }
        }
        pub mod async_ {
            use cornucopia_async::GenericClient;
            use futures;
            use futures::{StreamExt, TryStreamExt};
            pub struct UserQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                query: &'static str,
                extractor: fn(&tokio_postgres::Row) -> super::UserBorrowed,
                mapper: fn(super::UserBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> UserQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::UserBorrowed) -> R,
                ) -> UserQuery<'a, C, R, N> {
                    UserQuery {
                        client: self.client,
                        params: self.params,
                        query: self.query,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.client.prepare(self.query).await?;
                    let row = self.client.query_one(&stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.client.prepare(self.query).await?;
                    Ok(self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.client.prepare(self.query).await?;
                    let it = self
                        .client
                        .query_raw(&stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct PostQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                query: &'static str,
                extractor: fn(&tokio_postgres::Row) -> super::PostBorrowed,
                mapper: fn(super::PostBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PostQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::PostBorrowed) -> R,
                ) -> PostQuery<'a, C, R, N> {
                    PostQuery {
                        client: self.client,
                        params: self.params,
                        query: self.query,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.client.prepare(self.query).await?;
                    let row = self.client.query_one(&stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.client.prepare(self.query).await?;
                    Ok(self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.client.prepare(self.query).await?;
                    let it = self
                        .client
                        .query_raw(&stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct CommentQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                query: &'static str,
                extractor: fn(&tokio_postgres::Row) -> super::CommentBorrowed,
                mapper: fn(super::CommentBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> CommentQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::CommentBorrowed) -> R,
                ) -> CommentQuery<'a, C, R, N> {
                    CommentQuery {
                        client: self.client,
                        params: self.params,
                        query: self.query,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.client.prepare(self.query).await?;
                    let row = self.client.query_one(&stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.client.prepare(self.query).await?;
                    Ok(self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.client.prepare(self.query).await?;
                    let it = self
                        .client
                        .query_raw(&stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct SelectComplexQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                query: &'static str,
                extractor: fn(&tokio_postgres::Row) -> super::SelectComplexBorrowed,
                mapper: fn(super::SelectComplexBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SelectComplexQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::SelectComplexBorrowed) -> R,
                ) -> SelectComplexQuery<'a, C, R, N> {
                    SelectComplexQuery {
                        client: self.client,
                        params: self.params,
                        query: self.query,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.client.prepare(self.query).await?;
                    let row = self.client.query_one(&stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.client.prepare(self.query).await?;
                    Ok(self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.client.prepare(self.query).await?;
                    let it = self
                        .client
                        .query_raw(&stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub fn users() -> UsersStmt {
                UsersStmt("SELECT * FROM users")
            }
            pub struct UsersStmt(&'static str);
            impl UsersStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a self,
                    client: &'a C,
                ) -> UserQuery<'a, C, super::User, 0> {
                    UserQuery {
                        client,
                        params: [],
                        query: self.0,
                        extractor: |row| super::UserBorrowed {
                            id: row.get(0),
                            name: row.get(1),
                            hair_color: row.get(2),
                        },
                        mapper: |it| <super::User>::from(it),
                    }
                }
            }
            pub fn insert_user() -> InsertUserStmt {
                InsertUserStmt("INSERT INTO users (name, hair_color) VALUES ($1, $2)")
            }
            pub struct InsertUserStmt(&'static str);
            impl InsertUserStmt {
                pub async fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::StringSql,
                >(
                    &'a self,
                    client: &'a C,
                    name: &'a T1,
                    hair_color: &'a Option<T2>,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = client.prepare(self.0).await?;
                    client.execute(&stmt, &[name, hair_color]).await
                }
            }
            impl<
                    'a,
                    C: GenericClient + Send + Sync,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::StringSql,
                >
                cornucopia_async::Params<
                    'a,
                    super::InsertUserParams<T1, T2>,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for InsertUserStmt
            {
                fn params(
                    &'a self,
                    client: &'a C,
                    params: &'a super::InsertUserParams<T1, T2>,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.name, &params.hair_color))
                }
            }
            pub fn posts() -> PostsStmt {
                PostsStmt("SELECT * FROM posts")
            }
            pub struct PostsStmt(&'static str);
            impl PostsStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a self,
                    client: &'a C,
                ) -> PostQuery<'a, C, super::Post, 0> {
                    PostQuery {
                        client,
                        params: [],
                        query: self.0,
                        extractor: |row| super::PostBorrowed {
                            id: row.get(0),
                            user_id: row.get(1),
                            title: row.get(2),
                            body: row.get(3),
                        },
                        mapper: |it| <super::Post>::from(it),
                    }
                }
            }
            pub fn post_by_user_ids() -> PostByUserIdsStmt {
                PostByUserIdsStmt("SELECT * FROM posts WHERE user_id = ANY($1)")
            }
            pub struct PostByUserIdsStmt(&'static str);
            impl PostByUserIdsStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
                    &'a self,
                    client: &'a C,
                    ids: &'a T1,
                ) -> PostQuery<'a, C, super::Post, 1> {
                    PostQuery {
                        client,
                        params: [ids],
                        query: self.0,
                        extractor: |row| super::PostBorrowed {
                            id: row.get(0),
                            user_id: row.get(1),
                            title: row.get(2),
                            body: row.get(3),
                        },
                        mapper: |it| <super::Post>::from(it),
                    }
                }
            }
            pub fn comments() -> CommentsStmt {
                CommentsStmt("SELECT * FROM comments")
            }
            pub struct CommentsStmt(&'static str);
            impl CommentsStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a self,
                    client: &'a C,
                ) -> CommentQuery<'a, C, super::Comment, 0> {
                    CommentQuery {
                        client,
                        params: [],
                        query: self.0,
                        extractor: |row| super::CommentBorrowed {
                            id: row.get(0),
                            post_id: row.get(1),
                            text: row.get(2),
                        },
                        mapper: |it| <super::Comment>::from(it),
                    }
                }
            }
            pub fn comments_by_post_id() -> CommentsByPostIdStmt {
                CommentsByPostIdStmt("SELECT * FROM comments WHERE post_id = ANY($1)")
            }
            pub struct CommentsByPostIdStmt(&'static str);
            impl CommentsByPostIdStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
                    &'a self,
                    client: &'a C,
                    ids: &'a T1,
                ) -> CommentQuery<'a, C, super::Comment, 1> {
                    CommentQuery {
                        client,
                        params: [ids],
                        query: self.0,
                        extractor: |row| super::CommentBorrowed {
                            id: row.get(0),
                            post_id: row.get(1),
                            text: row.get(2),
                        },
                        mapper: |it| <super::Comment>::from(it),
                    }
                }
            }
            pub fn select_complex() -> SelectComplexStmt {
                SelectComplexStmt("SELECT u.id as myuser_id, u.name, u.hair_color, p.id as post_id, p.user_id, p.title, p.body FROM users as u LEFT JOIN posts as p on u.id = p.user_id")
            }
            pub struct SelectComplexStmt(&'static str);
            impl SelectComplexStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a self,
                    client: &'a C,
                ) -> SelectComplexQuery<'a, C, super::SelectComplex, 0> {
                    SelectComplexQuery {
                        client,
                        params: [],
                        query: self.0,
                        extractor: |row| super::SelectComplexBorrowed {
                            myuser_id: row.get(0),
                            name: row.get(1),
                            hair_color: row.get(2),
                            post_id: row.get(3),
                            user_id: row.get(4),
                            title: row.get(5),
                            body: row.get(6),
                        },
                        mapper: |it| <super::SelectComplex>::from(it),
                    }
                }
            }
        }
    }
}
