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

// pub fn establish_connection() -> SqliteConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     SqliteConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

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

graphql_object!(Database: Database as "Query" |&self| {
    description: "The root query object of the schema"

    field issue(
        id: String as "the issue id"
    ) -> FieldResult<graphql::models::Issue> {
        let db = self.pool.get()?;
        
        Ok(graphql::models::Issue {
            id: 42,
            title: "Test".to_owned(),
            description: "Hahha".to_owned()
        })
    }

    field issues(
        project_id: String as "the project id"
    ) -> FieldResult<Vec<graphql::models::Issue>> {
        use schema::issues::dsl::*;

        let db = self.pool.get()?;
        let records: Vec<models::Issue> = schema::issues::table.load(&*db).expect("Result!");
        let result: Vec<graphql::models::Issue> = records
            .into_iter()
            .map(|r| graphql::models::Issue::from_model(r))
            .collect();
        Ok(result)
    }
});

type Schema = RootNode<'static, Database, EmptyMutation<Database>>;

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

    // let new_issue = models::IssueBuilder::default()
    //     .id(0)
    //     .project_id(0)
    //     .issue_type(0)
    //     .status_id(0)
    //     .category_id(0)
    //     .created_by_user_id(0)
    //     .created_at(0)
    //     .title("Hello".to_string())
    //     .description("Haha.".to_string())
    //     .build()
    //     .unwrap();
    
    // diesel::insert_into(schema::issues::table)
    //     .values(&new_issue)
    //     .execute(&conn)
    //     .expect("Error saving new issue");

    rocket::ignite()
        .manage(Database::new())
        .manage(Schema::new(
            Database::new(),
            EmptyMutation::<Database>::new(),
        ))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
