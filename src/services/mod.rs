use chrono::{DateTime, Utc};
use mongodb::{bson::doc, error::Result, options::ClientOptions, Client, Collection, Database};

#[derive(Clone)]
pub struct ParticipantsRepository {
    confirmations: Collection<ConfirmationInfo>,
}

impl ParticipantsRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            confirmations: database.collection("Participants"),
        }
    }

    pub async fn upsert(&self, conf: ConfirmationInfo) -> Result<()> {
        let filter = doc! {"name": &conf.name};
        let info = self.confirmations.find_one(filter.clone(), None).await?;

        if info.is_some() {
            let update = doc! { "$set": { "escorts": conf.escorts } };
            self.confirmations.update_one(filter, update, None).await?;
        } else {
            self.confirmations.insert_one(conf, None).await?;
        }

        Ok(())
    }

    pub async fn get_info(&self, name: &str) -> Result<Option<ConfirmationInfo>> {
        let transactions = self
            .confirmations
            .find_one(doc! { "name": name }, None)
            .await?;

        Ok(transactions)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ConfirmationInfo {
    pub time: DateTime<Utc>,
    pub name: String,
    pub escorts: Option<Vec<String>>,
}

pub async fn get_mongo_client() -> Result<Client> {
    dotenv::dotenv().ok();

    let connection_string = std::env::var("MONGO_CONNECTION_STRING")
        .unwrap_or_else(|_| "mongodb://localhost/?retryWrites=true".to_string());

    let options = ClientOptions::parse(connection_string).await?;

    Client::with_options(options)
}
