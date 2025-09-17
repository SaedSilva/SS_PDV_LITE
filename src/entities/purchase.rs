use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Purchase {
    pub id: i64,
    pub total: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Purchase {
    pub fn new(id: i64, total: i64, created_at: NaiveDateTime) -> Self {
        Self {
            id,
            total,
            created_at,
            updated_at: None,
        }
    }
}
