create table "user" (

    user_id         uuid primary key                default gen_random_uuid(),

    username        text                not null,

    email           text                not null,

    password_hash   text                not null,

    created_at      timestamptz         not null    default now(),

    updated_at      timestamptz
);

SELECT trigger_updated_at('"user"');