table! {
    categories (id) {
        id -> Integer,
        category_name -> Text,
    }
}

table! {
    issue_assignees (issue_id, user_id) {
        issue_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    issue_comments (id) {
        id -> Integer,
        issue_id -> Integer,
        user_id -> Integer,
        comment -> Text,
        created_at -> Integer,
        modified_at -> Nullable<Integer>,
    }
}

table! {
    issue_histories (issue_id, project_id) {
        issue_id -> Integer,
        project_id -> Integer,
        ts -> Integer,
        event_type -> Integer,
    }
}

table! {
    issue_labels (issue_id, label_id) {
        issue_id -> Integer,
        label_id -> Integer,
    }
}

table! {
    issues (id, project_id) {
        id -> Integer,
        project_id -> Integer,
        issue_type -> Integer,
        created_at -> Integer,
        created_by_user_id -> Integer,
        status_id -> Integer,
        category_id -> Nullable<Integer>,
        title -> Text,
        description -> Text,
    }
}

table! {
    labels (id) {
        id -> Integer,
        label -> Text,
    }
}

table! {
    project_users (project_id, user_id) {
        project_id -> Integer,
        user_id -> Integer,
        role -> Text,
    }
}

table! {
    projects (id) {
        id -> Integer,
        slug -> Text,
        name -> Text,
    }
}

table! {
    statuses (id) {
        id -> Integer,
        status_name -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        iterations -> Integer,
        salt -> Binary,
        credential -> Binary,
    }
}

joinable!(issue_assignees -> users (user_id));
joinable!(issue_comments -> users (user_id));
joinable!(issue_labels -> labels (label_id));
joinable!(issues -> categories (category_id));
joinable!(issues -> projects (project_id));
joinable!(issues -> statuses (status_id));
joinable!(issues -> users (created_by_user_id));
joinable!(project_users -> projects (project_id));
joinable!(project_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    categories,
    issue_assignees,
    issue_comments,
    issue_histories,
    issue_labels,
    issues,
    labels,
    project_users,
    projects,
    statuses,
    users,
);
