use iso8061_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use crate::embed::author::EmbedAuthor;
use crate::embed::field::EmbedField;
use crate::embed::footer::EmbedFooter;
use crate::embed::image::EmbedImage;
use crate::embed::provider::EmbedProvider;
use crate::embed::thumbnail::EmbedThumbnail;
use crate::embed::video::EmbedVideo;

pub mod author;
pub mod field;
pub mod footer;
pub mod image;
pub mod provider;
pub mod thumbnail;
pub mod video;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<Timestamp>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

impl Embed {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn timestamp(mut self, timestamp: Timestamp) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn footer<Func>(mut self, function: Func) -> Self
    where
        Func: Fn(EmbedFooter) -> EmbedFooter,
    {
        let footer = function(EmbedFooter::new());

        self.footer = Some(footer);
        self
    }

    pub fn image<Func>(mut self, function: Func) -> Self
    where
        Func: Fn(EmbedImage) -> EmbedImage,
    {
        let image = function(EmbedImage::new());

        self.image = Some(image);
        self
    }

    pub fn thumbnail<Func>(mut self, function: Func) -> Self
    where
        Func: Fn(EmbedThumbnail) -> EmbedThumbnail,
    {
        let thumbnail = function(EmbedThumbnail::new());

        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn video<Func>(mut self, function: Func) -> Self
    where
        Func: Fn(EmbedVideo) -> EmbedVideo,
    {
        let video = function(EmbedVideo::new());

        self.video = Some(video);
        self
    }

    pub fn provider<Func>(mut self, function: Func) -> Self
    where
        Func: Fn(EmbedProvider) -> EmbedProvider,
    {
        let provider = function(EmbedProvider::new());

        self.provider = Some(provider);
        self
    }

    pub fn author<Func>(mut self, function: Func) -> Self
    where
        Func: Fn(EmbedAuthor) -> EmbedAuthor,
    {
        let author = function(EmbedAuthor::new());

        self.author = Some(author);
        self
    }

    pub fn field<Func>(mut self, function: Func) -> Self
    where
        Func: Fn(EmbedField) -> EmbedField,
    {
        let field = function(EmbedField::new());

        match self.fields {
            None => { self.fields = Some(vec![field]) }
            Some(ref mut v) => { v.push(field) }
        }
        self
    }
}
