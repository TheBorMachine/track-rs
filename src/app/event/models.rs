use clickhouse::{Client, Row, error::Error};
use serde::{Deserialize, Serialize};

#[derive(Row, Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    id: String,
    url: String,
    session: String,
    event_type: String,
    // timestamp: String,
}

pub async fn add_event(event: &Event, client: &Client) -> Result<(), Error> {
    let mut inserter = client.insert::<Event>("events")?;
    inserter.write(event).await?;
    inserter.end().await
}

pub async fn add_events(events: Vec<Event>, client: &Client) -> Result<(), Error> {
    let mut inserter = client.insert::<Event>("events")?;

    for event in events {
        inserter.write(&event).await?;
    }

    inserter.end().await
}
