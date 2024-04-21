use std::collections::HashMap;

pub struct InMemoryDatabase {
    inner: HashMap<String, ShoppingItem>,
}

struct ShoppingList {
    list: HashMap<String, ShoppingItem>,
}

impl Default for ShoppingList {
    fn default() -> Self {
        Self {
            list: [
                (
                    "6855cfc9-78fd-4b66-8671-f3c90ac2abd8".to_string(),
                    ShoppingItem {
                        title: "Coffee".to_string(),
                        creator: "Roland".to_string(),
                    },
                ),
                (
                    "3d778d1c-5a4e-400f-885d-10212027382d".to_string(),
                    ShoppingItem {
                        title: "Tomato Seeds".to_string(),
                        creator: "Tania".to_string(),
                    },
                ),
            ]
            .into(),
        }
    }
}

#[derive(Clone)]
pub struct ShoppingItem {
    pub title: String,
    pub creator: String,
}

impl InMemoryDatabase {
    pub fn get_item(&self, uuid: &str) -> Option<&ShoppingItem> {
        self.inner.get(uuid)
    }

    pub fn insert_item(&mut self, uuid: &str, item: ShoppingItem) {
        self.inner.insert(uuid.to_string(), item);
    }

    pub fn delete_item(&mut self, uuid: &str) {
        self.inner.remove(uuid);
    }

    pub fn as_vec(&self) -> Vec<(String, ShoppingItem)> {
        self.inner
            .iter()
            .map(|(uuid, item)| (uuid.clone(), item.clone()))
            .collect()
    }
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        let inner: HashMap<String, ShoppingItem> = [
            (
                "b8906da9-0c06-45a7-b117-357b784a8612".to_string(),
                ShoppingItem {
                    title: "Salt".to_string(),
                    creator: "Yasin".to_string(),
                },
            ),
            (
                "ac18131a-c7b8-4bdc-95b5-e1fb6cad4576".to_string(),
                ShoppingItem {
                    title: "Milk".to_string(),
                    creator: "Tim".to_string(),
                },
            ),
        ]
        .into_iter()
        .collect();

        Self { inner }
    }
}
