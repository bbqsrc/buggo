#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(trivial_bounds)]

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
#[macro_use]
extern crate juniper_relay;
extern crate base64;
extern crate ring;

use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use rocket::response::content;
use rocket::State;
use juniper::{FieldResult, RootNode};

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

pub struct Context {
    pool: SqlitePool
}
impl Context {
    pub fn new() -> Context {
        Context {
            pool: establish_pool()
        }
    }
}
impl juniper::Context for Context {}

struct Database;
impl Database {
    pub fn new() -> Database {
        Database {}
    }
}

struct DatabaseMutator;
impl DatabaseMutator {
    pub fn new() -> DatabaseMutator {
        DatabaseMutator {}
    }
}

use graphql::models::{
    ProjectConnection,
    ProjectEdge,
    UserConnection,
    UserEdge
};

graphql_object!(DatabaseMutator: Context as "Mutator" |&self| {
    description: "Mutation"

    field create_user(
        &executor,
        username: String,
        password: String
    ) -> FieldResult<graphql::models::User> {
        use schema::users::dsl as users;
        let username = username.to_lowercase();

        let db = executor.context().pool.get()?;
        let new_user = models::NewUser::new(username, password, 10000, 16);

        insert_into(schema::users::table)
            .values(&new_user)
            .execute(&*db)?;

        let record = schema::users::table
            .filter(users::username.eq(&new_user.username))
            .get_result(&*db)?;
        
        Ok(graphql::models::User::new(record))
    }

    field create_project(
        &executor,
        project_id: String as "URL slug to use for new project",
        name: String as "Project name"
    ) -> FieldResult<graphql::models::Project> {
        use schema::projects::dsl as projects;

        let db = executor.context().pool.get()?;
        let new_project = models::NewProject::new(project_id, name);

        insert_into(schema::projects::table)
            .values(&new_project)
            .execute(&*db)?;

        let record = schema::projects::table
            .filter(projects::slug.eq(&new_project.slug))
            .get_result(&*db)?;

        Ok(graphql::models::Project::new(record))
    }
});

graphql_object!(Database: Context as "Query" |&self| {
    description: "The root query object of the schema"

    field issue(
        &executor,
        project_id: String as "project id associated with issue",
        issue_id: i32 as "the issue id"
    ) -> FieldResult<graphql::models::Issue> {
        use schema::issues::dsl as issues;
        use schema::projects::dsl as projects;

        let db = executor.context().pool.get()?;

        let project: models::Project = schema::projects::table
            .filter(projects::slug.eq(project_id))
            .get_result(&*db)?;

        let issue: models::Issue = schema::issues::table
            .filter(issues::id.eq(issue_id)
                .and(issues::project_id.eq(&project.id)))
            .get_result(&*db)?;
            
        Ok(graphql::models::Issue::from_model(&project, &issue))
    }

    field project(&executor, id: String) -> FieldResult<Option<graphql::models::Project>> {
        use schema::projects::dsl as projects;

        let db = executor.context().pool.get()?;

        let project: QueryResult<models::Project> = schema::projects::table
            .filter(projects::slug.eq(&id))
            .get_result(&*db);
        
        let record = match project {
            Ok(v) => v,
            Err(_) => return Ok(None)
        };

        Ok(Some(graphql::models::Project::new(record)))
    }

    field users(&executor, first: i32, after: Option<String>) -> FieldResult<UserConnection> {
        use schema::users::dsl as users;
        use std::str;

        let db = executor.context().pool.get()?;

        let query = schema::users::table
            .order_by(users::username)
            .limit(first as i64);

        let users: Vec<models::User> = if let Some(v) = after {
            let cursor_id = base64::decode_config(&v, base64::URL_SAFE)?;
            let cursor_id = str::from_utf8(&cursor_id)?;
            query.filter(users::username.gt(&cursor_id)).get_results(&*db)
        } else {
            query.get_results(&*db)
        }?;

        let result: Vec<_> = users.into_iter()
            .map(|p| {
                let cursor_id = base64::encode_config(&p.username, base64::URL_SAFE);
                UserEdge::new(
                    graphql::models::User::new(p),
                    cursor_id
                )
            })
            .collect();

        let conn = UserConnection::new(
            juniper_relay::PageInfo {
                has_previous_page: false,
                has_next_page: false
            },
            result
        );

        Ok(conn)
    }

    field projects(&executor, first: i32, after: Option<String>) -> FieldResult<ProjectConnection> {
        use schema::projects::dsl as projects;
        use std::str;

        let db = executor.context().pool.get()?;

        let query = schema::projects::table
            .order_by(projects::slug)
            .limit(first as i64);

        let projects: Vec<models::Project> = if let Some(v) = after {
            let cursor_id = base64::decode_config(&v, base64::URL_SAFE)?;
            let cursor_id = str::from_utf8(&cursor_id)?;
            query.filter(projects::slug.gt(&cursor_id)).get_results(&*db)
        } else {
            query.get_results(&*db)
        }?;

        let result: Vec<_> = projects.into_iter()
            .map(|p| {
                let cursor_id = base64::encode_config(&p.slug, base64::URL_SAFE);
                ProjectEdge::new(
                    graphql::models::Project::new(p),
                    cursor_id
                )
            })
            .collect();

        let conn = ProjectConnection::new(
            juniper_relay::PageInfo {
                has_previous_page: false,
                has_next_page: false
            },
            result
        );

        Ok(conn)
    }
});

type Schema = RootNode<'static, Database, DatabaseMutator>;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Context::new())
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
