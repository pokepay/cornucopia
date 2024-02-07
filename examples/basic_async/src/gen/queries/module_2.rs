// This file was generated with `cornucopia`. Do not modify.

#![allow(clippy::all, clippy::pedantic)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#[derive(Debug)]
pub struct AuthorNameStartingWithParams<T1: cornucopia_async::StringSql> {
    pub start_str: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Authors {
    pub id: i32,
    pub name: String,
    pub country: String,
}
pub struct AuthorsBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub country: &'a str,
}
impl<'a> From<AuthorsBorrowed<'a>> for Authors {
    fn from(AuthorsBorrowed { id, name, country }: AuthorsBorrowed<'a>) -> Self {
        Self {
            id,
            name: name.into(),
            country: country.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AuthorNameStartingWith {
    pub authorid: i32,
    pub name: String,
    pub bookid: i32,
    pub title: String,
}
pub struct AuthorNameStartingWithBorrowed<'a> {
    pub authorid: i32,
    pub name: &'a str,
    pub bookid: i32,
    pub title: &'a str,
}
impl<'a> From<AuthorNameStartingWithBorrowed<'a>> for AuthorNameStartingWith {
    fn from(
        AuthorNameStartingWithBorrowed {
            authorid,
            name,
            bookid,
            title,
        }: AuthorNameStartingWithBorrowed<'a>,
    ) -> Self {
        Self {
            authorid,
            name: name.into(),
            bookid,
            title: title.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SelectTranslations {
    pub title: String,
    pub translations: Vec<String>,
}
pub struct SelectTranslationsBorrowed<'a> {
    pub title: &'a str,
    pub translations: cornucopia_async::ArrayIterator<'a, &'a str>,
}
impl<'a> From<SelectTranslationsBorrowed<'a>> for SelectTranslations {
    fn from(
        SelectTranslationsBorrowed {
            title,
            translations,
        }: SelectTranslationsBorrowed<'a>,
    ) -> Self {
        Self {
            title: title.into(),
            translations: translations.map(|v| v.into()).collect(),
        }
    }
}
use cornucopia_async::GenericClient;
use futures_util::{StreamExt, TryStreamExt};
pub struct AuthorsQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'a tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> AuthorsBorrowed,
    mapper: fn(AuthorsBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> AuthorsQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(AuthorsBorrowed) -> R) -> AuthorsQuery<'a, C, R, N> {
        AuthorsQuery {
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
pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'a tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> &str,
    mapper: fn(&str) -> T,
}
impl<'a, C, T: 'a, const N: usize> StringQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'a, C, R, N> {
        StringQuery {
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
pub struct AuthorNameStartingWithQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'a tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> AuthorNameStartingWithBorrowed,
    mapper: fn(AuthorNameStartingWithBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> AuthorNameStartingWithQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(AuthorNameStartingWithBorrowed) -> R,
    ) -> AuthorNameStartingWithQuery<'a, C, R, N> {
        AuthorNameStartingWithQuery {
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
pub struct PublicVoiceactorQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'a tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> super::super::types::public::VoiceactorBorrowed,
    mapper: fn(super::super::types::public::VoiceactorBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> PublicVoiceactorQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(super::super::types::public::VoiceactorBorrowed) -> R,
    ) -> PublicVoiceactorQuery<'a, C, R, N> {
        PublicVoiceactorQuery {
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
pub struct SelectTranslationsQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'a tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> SelectTranslationsBorrowed,
    mapper: fn(SelectTranslationsBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> SelectTranslationsQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(SelectTranslationsBorrowed) -> R,
    ) -> SelectTranslationsQuery<'a, C, R, N> {
        SelectTranslationsQuery {
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
pub fn authors() -> AuthorsStmt {
    AuthorsStmt(
        "SELECT
    *
FROM
    Author",
        None,
    )
}
pub struct AuthorsStmt(&'static str, Option<tokio_postgres::Statement>);
impl AuthorsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'a, C: GenericClient>(&'a self, client: &'a C) -> AuthorsQuery<'a, C, Authors, 0> {
        AuthorsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| AuthorsBorrowed {
                id: row.get(0),
                name: row.get(1),
                country: row.get(2),
            },
            mapper: |it| <Authors>::from(it),
        }
    }
}
pub fn books() -> BooksStmt {
    BooksStmt(
        "SELECT
    Title
FROM
    Book",
        None,
    )
}
pub struct BooksStmt(&'static str, Option<tokio_postgres::Statement>);
impl BooksStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'a, C: GenericClient>(&'a self, client: &'a C) -> StringQuery<'a, C, String, 0> {
        StringQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| row.get(0),
            mapper: |it| it.into(),
        }
    }
}
pub fn author_name_by_id() -> AuthorNameByIdStmt {
    AuthorNameByIdStmt(
        "SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = $1",
        None,
    )
}
pub struct AuthorNameByIdStmt(&'static str, Option<tokio_postgres::Statement>);
impl AuthorNameByIdStmt {
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
        id: &'a i32,
    ) -> StringQuery<'a, C, String, 1> {
        StringQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| row.get(0),
            mapper: |it| it.into(),
        }
    }
}
pub fn author_name_starting_with() -> AuthorNameStartingWithStmt {
    AuthorNameStartingWithStmt(
        "SELECT
    BookAuthor.AuthorId,
    Author.Name,
    BookAuthor.BookId,
    Book.Title
FROM
    BookAuthor
    INNER JOIN Author ON Author.id = BookAuthor.AuthorId
    INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
    Author.Name LIKE CONCAT($1::text, '%')",
        None,
    )
}
pub struct AuthorNameStartingWithStmt(&'static str, Option<tokio_postgres::Statement>);
impl AuthorNameStartingWithStmt {
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
        start_str: &'a T1,
    ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1> {
        AuthorNameStartingWithQuery {
            client,
            params: [start_str],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| AuthorNameStartingWithBorrowed {
                authorid: row.get(0),
                name: row.get(1),
                bookid: row.get(2),
                title: row.get(3),
            },
            mapper: |it| <AuthorNameStartingWith>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
    cornucopia_async::Params<
        'a,
        AuthorNameStartingWithParams<T1>,
        AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1>,
        C,
    > for AuthorNameStartingWithStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a AuthorNameStartingWithParams<T1>,
    ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith, 1> {
        self.bind(client, &params.start_str)
    }
}
pub fn select_voice_actor_with_character() -> SelectVoiceActorWithCharacterStmt {
    SelectVoiceActorWithCharacterStmt(
        "SELECT
    voice_actor
FROM
    SpongeBobVoiceActor
WHERE
    character = $1",
        None,
    )
}
pub struct SelectVoiceActorWithCharacterStmt(&'static str, Option<tokio_postgres::Statement>);
impl SelectVoiceActorWithCharacterStmt {
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
        spongebob_character: &'a super::super::types::public::SpongeBobCharacter,
    ) -> PublicVoiceactorQuery<'a, C, super::super::types::public::Voiceactor, 1> {
        PublicVoiceactorQuery {
            client,
            params: [spongebob_character],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| row.get(0),
            mapper: |it| it.into(),
        }
    }
}
pub fn select_translations() -> SelectTranslationsStmt {
    SelectTranslationsStmt(
        "SELECT
    Title,
    Translations
FROM
    Book",
        None,
    )
}
pub struct SelectTranslationsStmt(&'static str, Option<tokio_postgres::Statement>);
impl SelectTranslationsStmt {
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
    ) -> SelectTranslationsQuery<'a, C, SelectTranslations, 0> {
        SelectTranslationsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| SelectTranslationsBorrowed {
                title: row.get(0),
                translations: row.get(1),
            },
            mapper: |it| <SelectTranslations>::from(it),
        }
    }
}
