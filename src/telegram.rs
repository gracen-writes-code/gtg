use std::env;

use grammers_client::{types::{Chat, Dialog}, Client as GClient, Config};
use grammers_session::Session;
use serde_json::Value;

use tokio::runtime::Runtime as TokioRuntime;

const SECRETS: &'static str = include_str!("secrets.json");

#[derive(Debug)]
pub enum TelegramError {
    UnknownFailure
}

pub struct Client {
    rt: TokioRuntime,
    session_file: String,
    inner: GClient,
}

impl Client {
    pub fn new() -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let secrets: Value = serde_json::from_str(SECRETS).unwrap();
        let session_file = env::var("HOME").unwrap() + "/gtg.session";
        
        let client = rt.block_on(GClient::connect(Config {
            session: Session::load_file_or_create(session_file.clone()).unwrap(),
            api_id: secrets["api_id"].as_i64().unwrap() as i32,
            api_hash: secrets["api_hash"].as_str().unwrap().into(),
            params: Default::default(),
        })).unwrap();

        Self {
            rt,
            session_file,
            inner: client,
        }
    }

    pub fn logged_in(&self) -> Result<bool, TelegramError> {
        todo!()
    }

    pub fn get_user(&self) -> Result<User, TelegramError> {
        todo!()
    }

    pub fn save_session(&self) -> Result<(), TelegramError> {
        match self.inner.session().save_to_file(self.session_file) {
            Ok(()) => Ok(()),
            Err(_) => Err(TelegramError::UnknownFailure)
        }
    }
}