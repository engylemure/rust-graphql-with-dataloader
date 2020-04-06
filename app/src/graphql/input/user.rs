use validator::Validate;

#[derive(juniper::GraphQLInputObject, Debug, Validate, Deserialize)]
pub struct RegisterInput {
    /// Email for the User that is being registered
    #[validate(email(message = "This value should be a E-Mail"))]
    pub email: String,
    /// Name for the user. length constraints : (min = 1, max = 255)
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    /// Password for the authentication. length constraints : (min = 6)
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
