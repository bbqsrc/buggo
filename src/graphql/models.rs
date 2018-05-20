use ::models;

#[derive(GraphQLObject, Builder)]
#[graphql(description="An issue")]
pub struct Issue {
    pub id: i32,
    pub project_id: String,
    pub title: String,
    pub description: String
}

impl Issue {
    pub fn from_model(project: &models::Project, model: &models::Issue) -> Issue {
        IssueBuilder::default()
            .id(model.id)
            .project_id(project.slug.to_owned())
            .title(model.title.to_owned())
            .description(model.description.to_owned())
            .build()
            .expect("Model generation can never fail")
    }
}

#[derive(GraphQLObject, Builder)]
#[graphql(description="A project")]
pub struct Project {
    pub id: String
}

impl Project {
    pub fn from_model(project: &models::Project) -> Project {
        Project {
            id: project.slug.to_owned()
        }
    }
}