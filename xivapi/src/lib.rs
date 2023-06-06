use elasticsearch::{
    http::{headers::HeaderMap, request::JsonBody, transport::Transport, Method},
    Elasticsearch
};
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

static XIVAPI: &str = "https://xivapi.com/search";

pub struct Client {
    client: Elasticsearch,
    key: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        let transport = Transport::single_node(XIVAPI).unwrap();
        let client = Elasticsearch::new(transport);
        //TODO: Set up adding key as secret later
        let key = None;
        Self { client, key }
    }

    pub fn with_key(key: String) -> Self{
        let transport = Transport::single_node(XIVAPI).unwrap();
        let client = Elasticsearch::new(transport);
        Self{ client, key: Some(key) }
    }

    //TODO remove, we'll take Query as argument and process with internal methods instead.
    fn client(&self) -> &Elasticsearch{
        &self.client
    }
}

#[derive(Serialize, Debug)]
struct Query {
	indexes: Vec<String>,
    columns: Vec<String>,
    body: Vec<Filter>,
}

impl Into<Value> for Query{
    fn into(self) -> Value {
        json!(self)
    }
}

impl Into<JsonBody<Value>> for Query{
    fn into(self) -> JsonBody<Value> {
        JsonBody::new(self.into())
    }
}

#[derive(Serialize, Debug)]
enum Filter{
    Query,
    From(i32),
    Size(i32),
    Sort(String, String),
}

#[derive(Serialize, Debug)]
struct Range{
    from: (String, String),
    to: (String, String)
}

async fn get_item() -> String {
    let client = Client::new();
    let q = Query{
        indexes: vec!["item".to_string()],
        columns: vec!["ID".to_string(), "Name".to_string(), "Icon".to_string(), "LevelItem".to_string(), "LevelEquip".to_string(), "ItemSearchCategory.Name".to_string()],
        body: vec![Filter::Query, Filter::From(0), Filter::Size(2), Filter::Sort("LevelItem".to_string(), "desc".to_string())]
    };
    let query: JsonBody<serde_json::Value> = q.into();
    let request = client.client()
        .send(
            Method::Post,
            XIVAPI,
            HeaderMap::new(),
            Option::<&serde_json::Value>::None,
            Some(&query),
            None,
        )
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    format!("{:#}", request)
}

#[cfg(test)]
mod test {
    use super::*;

	#[tokio::test]
	async fn a() {
        //PLEASE remember to remove this, this is temporary test just for refactoring.
		assert_eq!(get_item().await, "{\"Pagination\":{\"Page\":1,\"PageNext\":null,\"PagePrev\":null,\"PageTotal\":1,\"Results\":2,\"ResultsPerPage\":100,\"ResultsTotal\":34},\"Results\":[{\"ID\":10713,\"Icon\":\"\\/i\\/040000\\/040215.png\",\"ItemSearchCategory\":{\"Name\":\"Head\"},\"LevelEquip\":52,\"LevelItem\":125,\"Name\":\"Wyvernskin Pot Helm of Maiming\"},{\"ID\":10720,\"Icon\":\"\\/i\\/043000\\/043266.png\",\"ItemSearchCategory\":{\"Name\":\"Body\"},\"LevelEquip\":52,\"LevelItem\":125,\"Name\":\"Holy Rainbow Shirt of Maiming\"}],\"SpeedMs\":4}".to_string());
	}
}

