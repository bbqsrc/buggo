use ::models;

#[derive(GraphQLObject, Builder)]
#[graphql(description="An issue")]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub description: String
}

impl Issue {
    pub fn from_model(model: models::Issue) -> Issue {
        IssueBuilder::default()
            .id(model.id)
            .title(model.title)
            .description(model.description)
            .build()
            .expect("Model generation can never fail")
    }
}