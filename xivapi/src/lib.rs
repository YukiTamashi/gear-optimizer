use elasticsearch::{
    http::{headers::HeaderMap, request::JsonBody, transport::Transport, Method},
    Elasticsearch,
};
use serde_json::json;

static XIV_API: &str = "https://xivapi.com/search";

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
        }
    )
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
    /*if let Ok(response) = request {
        response.text().await.unwrap()
    } else {
        "request failed".to_owned()
    }*/
    format!("{:?}", request)
    /* */
}
