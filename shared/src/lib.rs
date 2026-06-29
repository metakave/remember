use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VisionPayload {
    pub vision: String,
    pub mission: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoalsPayload {
    pub yearly: String,
    pub quarterly: String,
    pub monthly: String,
    pub weekly: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Reminder {
    pub id: Option<i64>,
    pub text: String,
    pub is_completed: bool,
    pub created_at: Option<String>,
}
