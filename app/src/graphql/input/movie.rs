use validator::Validate;

#[derive(juniper::GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct MovieFilter {
    pub id: Option<i32>,
    pub name: Option<String>,
}
