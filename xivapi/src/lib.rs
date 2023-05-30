use elasticsearch::{
    http::{headers::HeaderMap, request::JsonBody, transport::Transport, Method},
    Elasticsearch,
};
use serde_json::json;

pub struct Client {
    client: Elasticsearch,
}

impl Client {
    pub fn new() -> Self {
        let transport = Transport::single_node(XIV_API).unwrap();
        let client = Elasticsearch::new(transport);
        Self { client }
    }
}

static XIV_API: &str = "https://xivapi.com/search";

struct Query {
	indexes: Vec<String>,

}

async fn get_item() -> String {
    let transport = Transport::single_node(XIV_API).unwrap();
    let body: JsonBody<serde_json::Value> = json!(
        {
            "query": {
                "bool": {
                    "must": [
                        {
                            "wildcard": {
                            "NameCombined_en": "*aiming*"
                            }
                        }
                    ],
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
                        }
                    ]
                }
            },
    })
    .into();

    let client = Elasticsearch::new(transport);
    let request = client
        .send(
            Method::Post,
            XIV_API,
            HeaderMap::new(),
            Option::<&serde_json::Value>::None,
            Some(body),
            None,
        )
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    format!("{:?}", request)
}

#[cfg(test)]
mod test {
    use super::*;

	#[tokio::test]
	async fn a() {
		assert_eq!(get_item().await, "".to_string());
	}
}
