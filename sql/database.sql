create table "user" (
    "id" serial primary key,
    "name" text not null unique,
    "desc" text not null default '',
    "password" text not null default '',
    "email" text not null unique default '',
    "phone" text not null unique default '',
    "avatar_url" text not null default '',
    "level" int2 not null default 0,
    "status" int2 not null default 0,
    "identity" int2 not null default 0,
    "create_time" timestamp with time zone not null default now(),
    "update_time" timestamp with time zone not null default now()
);