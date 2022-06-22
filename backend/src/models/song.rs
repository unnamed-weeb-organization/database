use crate::utils::context::Context;

use super::{artist::Artist, release::Release, Name, NewName};
use juniper::graphql_object;
use sea_query::Iden;
use sqlx::{FromRow, postgres::PgRow, Row};
use ulid::Ulid;

#[derive(Clone, Debug)]

pub struct Song {
    pub id: Ulid,
    pub name: Name
}

pub enum SongIden {
    Table,
    Id,
    Name
}

impl Iden for SongIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                SongIden::Table => "songs",
                SongIden::Id => "id",
                SongIden::Name => "name"
            }
        )
        .unwrap();
    }
}

#[graphql_object(Context = Context)]
impl Song {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> &Name {
        &self.name
    }

    async fn artists(&self, context: &Context) -> Vec<Artist> {
        let db = &*context.db;
        crate::database::artist::get_artists_by_song_id(&self.id, db).await.unwrap()
    }

    async fn releases(&self, context: &Context) -> Vec<Release> {
        let db = &*context.db;
        crate::database::release::get_releases_by_song_id(&self.id, db).await.unwrap()
    }
}

impl<'r> FromRow<'r, PgRow> for Song {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get(0)?;
        let name: Name = row.try_get(1)?;

        Ok(Self {
            id: Ulid::from_string(&id).unwrap(),
            name,
        })
    }
}

#[derive(Clone, Debug, juniper::GraphQLInputObject)]
pub struct NewSong {
    pub name: NewName,
    pub artists: Vec<String>,
    pub releases: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub id: Option<String>,
    pub search: Option<String>,
    pub artist_id: Option<String>,
    pub release_id: Option<String>,
    pub genres: Option<Vec<String>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>
}