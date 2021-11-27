use std::collections::HashMap;

use async_trait::async_trait;
use futures::Stream;
use thiserror::Error;

use stream::{CreateMapper, DestroyMapper, GetMapper, StartMapper, StopMapper};

mod stream;

#[derive(Error, Debug)]
pub enum Error {
    #[error("There is no instance called: `{0}`")]
    InstanceMissing(String),
    #[error("IO Error: `{0}`")]
    IOError(#[from] std::io::Error),
    #[error("Unknown error")]
    Unknown,
}

type Result<T> = std::result::Result<T, Error>;

pub enum Status {
    Starting,
    Running,
    Failing,
    Offline,
}

pub struct InstanceInfo {
    pub name: String,
    pub wordpress_status: Status,
    pub database_status: Status,
    pub created: i64,
    pub labels: HashMap<String, String>,
}

pub struct CreateInfo {
    pub name: String,
    pub access_url: String,
}

#[async_trait]
pub trait MakepressManager: Sized + Clone {
    async fn get_all<T: AsRef<str>>(&self) -> Vec<Result<InstanceInfo>>;
    async fn get<T: AsRef<str>>(&self, name: T) -> Result<InstanceInfo>;
    async fn get_many<S>(&'static self, names: S) -> GetMapper<S, Self>
    where
        S: Stream + Send,
        S::Item: AsRef<str>,
    {
        GetMapper::new(self, names)
    }

    async fn create<T: AsRef<str>>(&self, name: T) -> Result<InstanceInfo>;
    async fn create_many<S>(&'static self, names: S) -> CreateMapper<S, Self>
    where
        S: Stream + Send,
        S::Item: AsRef<str>,
    {
        CreateMapper::new(self, names)
    }

    async fn start<T: AsRef<str>>(&self, name: T) -> Result<InstanceInfo>;
    async fn start_many<S>(&'static self, names: S) -> StartMapper<S, Self>
    where
        S: Stream + Send,
        S::Item: AsRef<str>,
    {
        StartMapper::new(self, names)
    }

    async fn stop<T: AsRef<str>>(&self, name: T) -> Result<InstanceInfo>;
    async fn stop_many<S>(&'static self, names: S) -> StopMapper<S, Self>
    where
        S: Stream + Send,
        S::Item: AsRef<str>,
    {
        StopMapper::new(self, names)
    }

    async fn destroy<T: AsRef<str>>(&self, name: T) -> Result<InstanceInfo>;
    async fn destroy_many<S>(&'static self, names: S) -> DestroyMapper<S, Self>
    where
        S: Stream + Send,
        S::Item: AsRef<str>,
    {
        DestroyMapper::new(self, names)
    }
}
