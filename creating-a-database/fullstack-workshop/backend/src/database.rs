use std::collections::HashMap;

pub struct InMemoryDatabase {
    inner: HashMap<String, ShoppingItem>,
}

struct ShoppingItem {
    title: String,
    creator: String,
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
