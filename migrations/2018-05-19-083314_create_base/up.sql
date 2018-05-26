CREATE TABLE projects (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  slug TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL
);

CREATE INDEX project_slugs ON projects (slug);

CREATE TABLE users (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL UNIQUE,
  iterations INTEGER NOT NULL,
  salt BLOB NOT NULL,
  credential BLOB NOT NULL
);

CREATE INDEX user_usernames ON users (username);

CREATE TABLE project_users (
  project_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  role TEXT NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects (id),
  FOREIGN KEY (user_id) REFERENCES users (id),
  PRIMARY KEY (project_id, user_id)
);

CREATE TABLE statuses (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  status_name TEXT NOT NULL
);

CREATE TABLE categories (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  category_name TEXT NOT NULL
);

CREATE TABLE issues (
  id INTEGER NOT NULL,
  project_id INTEGER NOT NULL,
  issue_type INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  created_by_user_id INTEGER NOT NULL,
  status_id INTEGER NOT NULL DEFAULT 0,
  category_id INTEGER,

  title TEXT NOT NULL,
  description TEXT NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects (id),
  FOREIGN KEY (created_by_user_id) REFERENCES users (id),
  FOREIGN KEY (category_id) REFERENCES categories (id),
  FOREIGN KEY (status_id) REFERENCES statuses (id),
  PRIMARY KEY (id, project_id)
);

CREATE TABLE issue_assignees (
  issue_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,

  FOREIGN KEY (issue_id) REFERENCES issues (id),
  FOREIGN KEY (user_id) REFERENCES users (id),
  PRIMARY KEY (issue_id, user_id)
);

CREATE TABLE labels (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  label TEXT NOT NULL UNIQUE
);

CREATE TABLE issue_labels (
  issue_id INTEGER NOT NULL,
  label_id INTEGER NOT NULL,

  FOREIGN KEY (issue_id) REFERENCES issues (id),
  FOREIGN KEY (label_id) REFERENCES labels (id),
  PRIMARY KEY (issue_id, label_id)
);

CREATE TABLE issue_comments (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  issue_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  comment TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  modified_at INTEGER,

  FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE issue_histories (
  issue_id INTEGER NOT NULL,
  project_id INTEGER NOT NULL,
  ts INTEGER NOT NULL,
  event_type INTEGER NOT NULL,

  FOREIGN KEY (issue_id) REFERENCES issues (id),
  FOREIGN KEY (project_id) REFERENCES issues (project_id),
  PRIMARY KEY (issue_id, project_id)
);