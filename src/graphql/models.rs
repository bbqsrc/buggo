use ::models;
use ::schema;
use ::Context;
use juniper::FieldResult;
use diesel::prelude::*;
use juniper_relay::PageInfo;

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

relay_connection!(IssueConnection, IssueEdge, Issue, Context);

pub struct Project {
    model: models::Project
}

graphql_object!(Project: Context as "Project" |&self| {
    description: "A bug tracking project"

    field id() -> &str {
        &self.model.slug
    }

    field issues(&executor, first: i32, cursor: Option<String>) -> FieldResult<IssueConnection> {
        use schema::issues::dsl as issues;

        let db = executor.context().pool.get()?;

        let records: Vec<models::Issue> = schema::issues::table
            .filter(issues::project_id.eq(&self.model.id))
            .load(&*db)
            .expect("Result!");
            
        let cursor_id = "test".to_string();
        let result = records
            .into_iter()
            .map(|r| IssueEdge::new(Issue::from_model(&self.model, &r), "?".to_owned()))
            .collect();

        let conn = IssueConnection {
            page_info: PageInfo {
                has_previous_page: false,
                has_next_page: false
            },
            edges: result
        };
        
        Ok(conn)
    }
});

impl Project {
    pub fn new(project: models::Project) -> Project {
        Project { model: project }
    }
}

relay_connection!(ProjectConnection, ProjectEdge, Project, Context);

pub struct User {
    model: models::User
}

impl User {
    pub fn new(user: models::User) -> User {
        User { model: user }
    }
}

graphql_object!(User: Context as "User" |&self| {
    description: "A user"

    field id() -> i32 {
        self.model.id
    }

    field username() -> &String {
        &self.model.username
    }
});

relay_connection!(UserConnection, UserEdge, User, Context);