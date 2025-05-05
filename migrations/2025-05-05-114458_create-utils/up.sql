create table utils (
    id serial primary key,
    name varchar(255) not null,
    ai_apikey varchar(255) not null,
    created_at timestamp default now()
);