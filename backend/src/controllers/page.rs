use juniper::{FieldResult};

use crate::{utils::context::Context, models::{song::Song, artist::Artist}};

#[derive(Clone, Debug)]
pub struct Page {
	pub page_info: PageInfo
}

#[graphql_object(context = Context)]
impl Page {
	async fn songs<'ctx>(&self, id: Option<String>, search: Option<String>, artist_id: Option<String>, release_id: Option<String>, genres: Option<Vec<String>>, context: &'ctx Context) -> FieldResult<Vec<Song>> {
        // Ok(Song)
        let db = &*context.db;
        let mut options = crate::models::song::Options {
            id: None,
            search: None,
            artist_id: None,
            release_id: None,
            genres: None,
			page: self.page_info.current_page,
			per_page: self.page_info.per_page,

        };

        if let Some(id) = id {
            options.id = Some(id);
        }

        if let Some(search) = search {
            options.search = Some(search);
        }

        if let Some(artist_id) = artist_id {
            options.artist_id = Some(artist_id);
        }

        if let Some(release_id) = release_id {
            options.release_id = Some(release_id);
        }

        if let Some(genres) = genres {
            options.genres = Some(genres);
        }

        Ok(crate::database::song::get_songs(&options, db).await.unwrap())
    }

    /// Get Artists
    async fn artists<'ctx>(&self, id: Option<String>, search: Option<String>, song_id: Option<String>, release_id: Option<String>, context: &'ctx Context) -> FieldResult<Vec<Artist>> {
        // Ok(Artist)
        let db = &*context.db;
        let mut options = crate::models::artist::Options {
            id: None,
            search: None,
            song_id: None,
            release_id: None,
            page: self.page_info.current_page,
            per_page: self.page_info.per_page,
        };

        if let Some(id) = id {
            options.id = Some(id);
        }

        if let Some(search) = search {
            options.search = Some(search);
        }

        if let Some(song_id) = song_id {
            options.song_id = Some(song_id);
        }

        if let Some(release_id) = release_id {
            options.release_id = Some(release_id);
        }

        Ok(crate::database::artist::get_artists(&options, db).await.unwrap())
    }
}


#[derive(Clone, Debug)]
pub struct PageInfo {
	pub total: i32,
	pub per_page: Option<i32>,
	pub current_page: Option<i32>,
	pub last_page: i32
}