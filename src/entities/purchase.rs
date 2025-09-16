use chrono::{Local, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct Purchase {
    pub id: i64,
    pub total: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Purchase {
    pub fn new(total: i64) -> Self {
        Self {
            id: 0, // Will be set by the database
            total,
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}
