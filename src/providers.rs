pub mod billa;
pub mod mpreis;
pub mod spar;
pub mod hofer;

use std::io::Write;
use reqwest::Client;
use clap::ValueEnum;
use crate::error::Result;

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum FetchSource {
    Billa,
    Hofer,
    Mpreis,
    Spar,
}

pub trait Fetchable {
    type ResponseImpl;

    async fn fetch(client: &Client) -> Result<Self::ResponseImpl>;
}
