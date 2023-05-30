use elasticsearch::{
    http::{headers::HeaderMap, request::JsonBody, transport::Transport, Method},
    Elasticsearch, SearchParts,
};
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
}


struct Query {
	indexes: Vec<String>,
    columns: Vec<String>,
    body: Vec<Filter>,
    from: Option<i32>,
    size: Option<i32>,
    sort: Option<String>,

}

enum Filter{}

async fn get_item() -> String {
    let transport = Transport::single_node(XIVAPI).unwrap();
    let query = json!(
        {   "body":{
            "query":{"bool":{
                "must": [
                    {
                    "wildcard": {
                        "NameCombined_en": "*aiming*"
                        }
                    }
                ]}},
            "from": "0",
            "size": "10",
            "sort": [
                    {
                        "LevelItem": "desc"
                    }
            ],
        },
            "indexes": "item",
            "columns": "Name, LevelItem"}
    );
    println!("{:#}", query);
    let query: JsonBody<serde_json::Value> = query.into();
    let client = Elasticsearch::new(transport);
    let request = client
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
		assert_eq!(get_item().await, "".to_string());
	}
}

/*"query":{
                "bool": {
                    
                    "filter": [
                        {
                            "range": {
                                "ItemSearchCategory.ID": {
                                    "gt": "1"
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
                        },
                    ],
                },
                "
            }, */