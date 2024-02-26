mod algolia;
pub mod billa;
mod factfinder;
pub mod hofer;
mod markant;
pub mod mpreis;
mod roksh;
pub mod spar;

use crate::error::Result;
use clap::ValueEnum;
use reqwest::Client;
use std::future::Future;
use std::io::Write;

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum FetchSource {
    Billa,
    Hofer,
    Mpreis,
    Spar,
}

pub trait Fetch {
    type ResponseImpl;
    const API_BASE_URL: &'static str;

    fn fetch(client: &Client) -> impl Future<Output = Result<Self::ResponseImpl>>;
}

pub trait Merge<Other = Self> {
    fn merge(&mut self, rhs: Other);
}
