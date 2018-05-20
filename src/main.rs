#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate derive_builder;
extern crate dotenv;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;
extern crate r2d2;
extern crate r2d2_diesel;

use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use rocket::response::content;
use rocket::State;
use juniper::{FieldResult, EmptyMutation, RootNode};

pub mod schema;
pub mod models;
mod graphql;

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_pool() -> SqlitePool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}

struct Database {
    pool: SqlitePool
}
impl Database {
    pub fn new() -> Database {
        Database {
            pool: establish_pool()
        }
    }
}

struct DatabaseMutator;
impl DatabaseMutator {
    pub fn new() -> DatabaseMutator {
        DatabaseMutator {}
    }
}

graphql_object!(DatabaseMutator: Database as "Mutator" |&self| {
    description: "Mutation"

    field create_project(
        &executor,
        project_id: String as "URL slug to use for new project"
    ) -> FieldResult<graphql::models::Project> {
        use schema::projects::dsl as projects;

        let db = executor.context().pool.get()?;
        let new_project = models::NewProject::new(project_id);

        insert_into(schema::projects::table)
            .values(&new_project)
            .execute(&*db)?;

        Ok(graphql::models::ProjectBuilder::default()
            .id(new_project.slug)
            .build()
            .unwrap())
    }
});

graphql_object!(Database: Database as "Query" |&self| {
    description: "The root query object of the schema"

    field issue(
        project_id: String as "project id associated with issue",
        issue_id: i32 as "the issue id"
    ) -> FieldResult<graphql::models::Issue> {
        use schema::issues::dsl as issues;
        use schema::projects::dsl as projects;

        let db = self.pool.get()?;

        let project: models::Project = schema::projects::table
            .filter(projects::slug.eq(project_id))
            .get_result(&*db)?;

        let issue: models::Issue = schema::issues::table
            .filter(issues::id.eq(issue_id)
                .and(issues::project_id.eq(&project.id)))
            .get_result(&*db)?;
            
        Ok(graphql::models::Issue::from_model(&project, &issue))
    }

    field issues(
        project_id: String as "the project id"
    ) -> FieldResult<Vec<graphql::models::Issue>> {
        use schema::issues::dsl as issues;
        use schema::projects::dsl as projects;

        let db = self.pool.get()?;

        let project: models::Project = schema::projects::table
            .filter(projects::slug.eq(project_id))
            .get_result(&*db)?;

        let records: Vec<models::Issue> = schema::issues::table
            .filter(issues::project_id.eq(&project.id))
            .load(&*db)
            .expect("Result!");
            
        let result: Vec<graphql::models::Issue> = records
            .into_iter()
            .map(|r| graphql::models::Issue::from_model(&project, &r))
            .collect();
        Ok(result)
    }

    field all_projects() -> FieldResult<Vec<graphql::models::Project>> {
        let db = self.pool.get()?;

        let projects: Vec<models::Project> = schema::projects::table
            .get_results(&*db)?;

        let result: Vec<_> = projects.into_iter()
            .map(|p| graphql::models::Project::from_model(&p))
            .collect();
        Ok(result)
    }
});

type Schema = RootNode<'static, Database, DatabaseMutator>;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Database::new())
        .manage(Schema::new(
            Database::new(),
            DatabaseMutator::new(),
        ))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
