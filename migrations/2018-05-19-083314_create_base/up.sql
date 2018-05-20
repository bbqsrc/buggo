CREATE TABLE projects (
  id INT NOT NULL PRIMARY KEY AUTOINCREMENT,
  slug TEXT NOT NULL UNIQUE
);

CREATE TABLE users (
  id INT NOT NULL PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL UNIQUE
);

CREATE TABLE statuses (
  id INT NOT NULL PRIMARY KEY AUTOINCREMENT,
  status_name TEXT NOT NULL
);

CREATE TABLE categories (
  id INT NOT NULL PRIMARY KEY AUTOINCREMENT,
  category_name TEXT NOT NULL
);

CREATE TABLE issues (
  id INT NOT NULL AUTOINCREMENT,
  project_id INT NOT NULL,
  issue_type INT NOT NULL,
  created_at INT NOT NULL,
  created_by_user_id INT NOT NULL,
  status_id INT NOT NULL DEFAULT 0,
  category_id INT,

  title TEXT NOT NULL,
  description TEXT NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects (id),
  FOREIGN KEY (created_by_user_id) REFERENCES users (id),
  FOREIGN KEY (category_id) REFERENCES categories (id),
  FOREIGN KEY (status_id) REFERENCES statuses (id),
  PRIMARY KEY (id, project_id)
);

CREATE TABLE issue_assignees (
  issue_id INT NOT NULL,
  user_id INT NOT NULL,

  FOREIGN KEY (issue_id) REFERENCES issues (id),
  FOREIGN KEY (user_id) REFERENCES users (id),
  PRIMARY KEY (issue_id, user_id)
);

CREATE TABLE labels (
  id INT PRIMARY KEY AUTOINCREMENT,
  label TEXT NOT NULL UNIQUE
);

CREATE TABLE issue_labels (
  issue_id INT NOT NULL,
  label_id INT NOT NULL,

  FOREIGN KEY (issue_id) REFERENCES issues (id),
  FOREIGN KEY (label_id) REFERENCES labels (id),
  PRIMARY KEY (issue_id, label_id)
);

CREATE TABLE issue_comments (
  id INT NOT NULL PRIMARY KEY AUTOINCREMENT,
  issue_id INT NOT NULL,
  user_id INT NOT NULL,
  comment TEXT NOT NULL,
  created_at INT NOT NULL,
  modified_at INT,

  FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE issue_histories (
  issue_id INT NOT NULL,
  project_id INT NOT NULL,
  ts INT NOT NULL,
  event_type INT NOT NULL,

  FOREIGN KEY (issue_id) REFERENCES issues (id),
  FOREIGN KEY (project_id) REFERENCES issues (project_id),
  PRIMARY KEY (issue_id, project_id)
);