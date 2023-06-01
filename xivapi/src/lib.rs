use elasticsearch::{
    http::{headers::HeaderMap, request::JsonBody, transport::Transport, Method},
    Elasticsearch, SearchParts,
};
use serde::Serialize;
use serde_json::json;

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

#[derive(Serialize)]
struct Query {
	indexes: Vec<String>,
    columns: Vec<String>,
    query: Vec<Filter>,
    from: Option<i32>,
    size: Option<i32>,
    sort: Option<(String, String)>,
    range: Option<Range>,
}

#[derive(Serialize)]
enum Filter{}

#[derive(Serialize)]
struct Range{
    from: (String, String),
    to: (String, String)
}

async fn get_item() -> String {
    let client = Client::new();
    let query = json!({
        "body":{
            "query":{
                "bool":{
                    "must": [{
                        "wildcard": {
                            "NameCombined_en": "*aiming*"
                        }
                    }],
                    "filter": [{
                        "range": {
                            "ItemSearchCategory.ID": {
                                "gt": 1
                                }
                            }
                    },
                    {
                        "range": {
                            "LevelItem": {
                                "gte": "100"
                                }
                            }
                        },
                        {
                        "range": {
                            "LevelItem": {
                                "lte": "125"
                                }
                            }
                        }
                    ]},    
            },
            "from": "0",
            "size": "2",
            "sort": [{
                "LevelItem": "desc"
                }],
        },
        "indexes": "item",
        "columns": "ID,Name,Icon,LevelItem,LevelEquip,ItemSearchCategory.Name"}
    );
    println!("{:#}", query);
    let query: JsonBody<serde_json::Value> = query.into();
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
		assert_eq!(get_item().await, "{\"Pagination\":{\"Page\":1,\"PageNext\":null,\"PagePrev\":null,\"PageTotal\":1,\"Results\":2,\"ResultsPerPage\":100,\"ResultsTotal\":34},\"Results\":[{\"ID\":10713,\"Icon\":\"\\/i\\/040000\\/040215.png\",\"ItemSearchCategory\":{\"Name\":\"Head\"},\"LevelEquip\":52,\"LevelItem\":125,\"Name\":\"Wyvernskin Pot Helm of Maiming\"},{\"ID\":10720,\"Icon\":\"\\/i\\/043000\\/043266.png\",\"ItemSearchCategory\":{\"Name\":\"Body\"},\"LevelEquip\":52,\"LevelItem\":125,\"Name\":\"Holy Rainbow Shirt of Maiming\"}],\"SpeedMs\":4}".to_string());
	}
}

