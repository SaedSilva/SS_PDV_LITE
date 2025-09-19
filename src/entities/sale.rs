use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Sale {
    pub id: i64,
    pub total: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Sale {
    pub fn new(id: i64, total: i64, created_at: NaiveDateTime) -> Self {
        Self {
            id,
            total,
            created_at,
            updated_at: None,
        }
    }
}
