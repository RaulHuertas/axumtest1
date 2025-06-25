

create table updates (

    -- automtically generated id and registration_date
    id serial primary key,
    registration_date timestamp with time zone not null default now(),

    -- update information
    version integer not null,
    description varchar not null,
    role varchar not null
);




