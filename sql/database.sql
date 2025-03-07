create table user(
    id int primary key,
    name text,
    possword test
)

create table post(
    id int primary key,
    title text,
    uploader int,
    contest text,
    class text,
    created text,
    update text,
    user text,
    foreign key (uploader) references "user" (id)
)