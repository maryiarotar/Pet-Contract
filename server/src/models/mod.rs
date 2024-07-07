use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    id: i32,
    first_name: String,
    last_name: String,
    nickname: String,
}


impl Author {
    pub fn new(
        id: i32,
        first_name: String,
        last_name: String,
        nickname: String,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            nickname,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub topic: String,
    pub file_path: String,
    pub description: String,
    pub author: String,
    pub date_published: String,
}

impl Project {
    // Метод для создания экземпляра Project из значений полей
    pub fn new(
        id: i32,
        name: String,
        topic: String,
        file_path: String,
        description: String,
        author: String,
        date_published: String,
    ) -> Self {
        Self {
            id,
            name,
            topic,
            file_path,
            description,
            author,
            date_published,
        }
    }
}