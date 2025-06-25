create table devices (
    -- automtically generated id and registration_date
    id serial primary key,
    registration_date timestamp with time zone not null default now(),

    -- device information 
    role varchar not null,
    phy_id varchar not null,
    description varchar not null,
    latest_version integer not null default 0,
    latest_updated_timestamp timestamp with time zone not null,

);



