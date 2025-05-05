
Create table tokens (
    id serial primary key,
    token varchar(255) not null,
    created_at timestamp default now()
);