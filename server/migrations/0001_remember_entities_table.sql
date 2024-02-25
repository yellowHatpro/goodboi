create table remember_entities (
    id varchar not null ,
    title varchar not null ,
    cmds text[],
    description varchar,
    pwd varchar not null
);