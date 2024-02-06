pub static DETECTIONS: &[&str] = &[
    "'; select sql from sqlite_schema --",
    "' or 1=1 --",
    "' union select sqlite_version() --",
    "' union select null,sqlite_version() --",
    "' union select null,null,sqlite_version() --",
];

pub static PROMPTS: &[&str] = &[
    "'; select null,null,version() --",
    "' union select null,null,sqlite_version() --",
    "' union select null,username,password from users --",
    "' union select name from sqlite_schema --",
    "' or 1=1 --",
];
