use reqwest::RequestBuilder;
use tokio::task;
use thiserror::Error;
use scraper::{Html, Selector, html::Select};

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("Error getting request response: {0}")]
    ResponseError(#[from] reqwest::Error),

    #[error("Element not found")]
    ElementNotFound,

    #[error("Element {0} not found")]
    ElementNotFoundWithId(&'static str), 

    #[error("Error joining task: {0}")]
    JoinError(#[from] task::JoinError)
}

pub struct Scraper {
    req: RequestBuilder,
    selector: Option<Selector>
}

impl Scraper {
    pub fn new(req: RequestBuilder) -> Self {
        Self { req, selector: None }
    }

    pub fn set_root_element(mut self, selector: &str) -> Self {
        self.selector = Some(Selector::parse(selector).unwrap());
        self
    }

    pub async fn get<F, R>(self, func: F) -> Result<R, ScraperError>
    where
        F: FnOnce(Select<'_, '_>) -> R + Send + 'static,
        R: Send + 'static,
    {
        let selector = self.selector.ok_or(ScraperError::ElementNotFound)?;

        let res = self.req.send().await.map_err(ScraperError::ResponseError)?;
        let body = res.text().await.map_err(ScraperError::ResponseError)?;

        task::spawn_blocking(move || {
            let document = Html::parse_document(&body);
            let select = document.select(&selector);
            func(select)
        })
        .await
        .map_err(ScraperError::JoinError)
    }

    // pub async fn get_by_attr(self, attr: &'static str) -> Result<String, ScraperError> {
    //     self.get(|mut select| {
    //         let element = select
    //             .next()
    //             .ok_or(ScraperError::ElementNotFound)?
    //             .value();

    //         element
    //             .attr(attr)
    //             .ok_or(ScraperError::AttributeNotFound)
    //             .map(|value| value.to_owned())
    //     }).await?
    // }
}