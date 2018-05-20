use super::schema::issues;

#[derive(Queryable, Insertable, Debug, Builder)]
#[table_name = "issues"]
pub struct Issue {
    pub id: i32,
    pub project_id: i32,
    pub issue_type: i32,// i16,
    pub created_at: i32,// i64
    pub created_by_user_id: i32,
    pub status_id: i32,// i16,
    pub category_id: Option<i32>,
    pub title: String,
    pub description: String,
}
