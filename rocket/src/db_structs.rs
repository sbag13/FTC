use crate::schema::users;

#[derive(Serialize, Insertable, Deserialize, Queryable)]
#[table_name="users"]
pub struct InsertableUser {
    pub mail: String,
    pub password: String,
}