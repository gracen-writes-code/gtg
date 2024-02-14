use grammers_client::{types::{Chat, Dialog}, Client as GClient, Config};
use grammers_session::Session;
use serde_json::Value;

use tokio::runtime::Runtime as TokioRuntime;

const SECRETS: &'static str = include_str!("secrets.json");

pub struct TelegramClient {
    rt: TokioRuntime,

    inner: GClient,
}

impl Client {
    fn new() -> Self {
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
            inner: client,
        }
    }
}