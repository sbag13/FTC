use crate::schema::users;

#[derive(Serialize, Insertable, Deserialize)]
#[table_name="users"]
pub struct InsertableUser {
    pub mail: String,
    pub password: String,
}