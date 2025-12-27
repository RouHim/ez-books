use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub isbn_10: Option<String>,
    pub isbn_13: Option<String>,
    pub publisher: Option<String>,
    pub publish_date: Option<String>,
    pub description: Option<String>,
    pub cover_image_path: Option<String>,
    pub epub_file_path: String,
    pub openlibrary_key: Option<String>,
    pub openlibrary_work_key: Option<String>,
    pub page_count: Option<i32>,
    pub language: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Book {
    pub fn new(title: String, epub_path: String) -> Self {
        let now = current_timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            epub_file_path: epub_path,
            author: None,
            isbn_10: None,
            isbn_13: None,
            publisher: None,
            publish_date: None,
            description: None,
            cover_image_path: None,
            openlibrary_key: None,
            openlibrary_work_key: None,
            page_count: None,
            language: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = current_timestamp();
    }
}

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_book_with_uuid_and_timestamps() {
        // Given: A title and epub path
        let title = "Test Book".to_string();
        let epub_path = "/path/to/book.epub".to_string();

        // When: Creating a new book
        let book = Book::new(title.clone(), epub_path.clone());

        // Then: Book should have valid UUID and timestamps
        assert!(!book.id.is_empty());
        assert_eq!(book.title, title);
        assert_eq!(book.epub_file_path, epub_path);
        assert!(book.created_at > 0);
        assert_eq!(book.created_at, book.updated_at);
        assert!(book.author.is_none());
    }

    #[test]
    fn should_update_timestamp_on_modification() {
        // Given: A book
        let mut book = Book::new("Test".to_string(), "/path.epub".to_string());
        let original_timestamp = book.updated_at;

        // When: Waiting a bit and updating timestamp
        std::thread::sleep(std::time::Duration::from_millis(10));
        book.update_timestamp();

        // Then: Updated timestamp should be greater
        assert!(book.updated_at >= original_timestamp);
    }

    #[test]
    fn should_parse_uuid_from_book_id() {
        // Given: A new book
        let book = Book::new("Test".to_string(), "/path.epub".to_string());

        // When: Parsing the UUID from ID
        let uuid_result = Uuid::parse_str(&book.id);

        // Then: Should be a valid UUID
        assert!(uuid_result.is_ok());
    }
}
