use super::models;

#[derive(Clone)]
pub struct Events {
    collection: mongodb::Collection<models::Event>,
}

impl Events {
    pub fn new(client: mongodb::Client) -> Self {
        let database = client.database("events_db");
        let collection = database.collection::<models::Event>("users");

        Self { collection }
    }

    pub async fn create(&self, event: &models::Event) {
        self.collection.insert_one(event).await.unwrap();
    }

    pub async fn create_many(&self, events: &[models::Event]) {
        self.collection.insert_many(events).await.unwrap();
    }
}
