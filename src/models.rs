use super::schema::{issues, projects, users};
use ring::{digest, pbkdf2};
use ring::rand::{SystemRandom, SecureRandom};

#[derive(Queryable, Insertable, Debug, Builder)]
#[table_name = "issues"]
pub struct Issue {
    pub id: i32,
    pub project_id: i32,
    pub issue_type: i32,// i16,
    pub created_at: i32,// i64
    pub created_by_user_id: i32,
    pub status_id: i32,// i16,
    pub category_id: Option<i32>,
    pub title: String,
    pub description: String,
}

#[derive(Queryable, Debug, Builder)]
pub struct Project {
    pub id: i32,
    pub slug: String,
    pub name: String
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct NewProject {
    pub slug: String,
    pub name: String
}

impl NewProject {
    pub fn new(slug: String, name: String) -> NewProject {
        NewProject {
            slug: slug,
            name: name
        }
    }
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub iterations: i32,
    pub salt: Vec<u8>,
    pub credential: Vec<u8>
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub iterations: i32,
    pub salt: Vec<u8>,
    pub credential: Vec<u8>
}

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

impl NewUser {
    pub fn new(username: String, password: String, iterations: u32, salt_len: usize) -> NewUser {
        let rand = SystemRandom::new();
        let mut salt = vec![0; salt_len];
        rand.fill(&mut salt).unwrap();
        
        let mut out: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(DIGEST_ALG, iterations, &salt, password.as_bytes(), &mut out);
        
        NewUser {
            username: username.to_lowercase(),
            iterations: iterations as i32,
            salt: salt,
            credential: out.to_vec()
        }
    }
}