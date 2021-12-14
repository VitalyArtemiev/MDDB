-- users
create type user_role as enum ('guest', 'user', 'admin');

create table users
(
    key   serial
        constraint users_pk
            primary key,
    login varchar(32) not null,
    role  user_role   not null,
    salt  varchar(32) not null,
    hash  varchar(128) not null
);

alter table users
    owner to medical;

create unique index users_login_uindex
    on users (login);

-- patients
create table patients
(
    id             serial
        constraint patients_pk
            primary key,
    first_name     varchar(32) not null,
    second_name    varchar(32),
    last_name      varchar(32) not null,
    dob            date        not null,
    address        varchar(64) not null,
    diagnosis      varchar(128),
    diagnosis_code varchar(16),
    occupation     varchar(32)
);

alter table patients
    owner to medical;

-- medical_institutions
create table medical_institutions
(
    id       serial
        constraint medical_institutions_pkey
            primary key,
    fullname varchar(64) not null
        constraint medical_institutions_fullname_key
            unique,
    address  varchar(64)
);

alter table medical_institutions
    owner to medical;

--documents
create table documents
(
    id              serial
        constraint document_pk
            primary key,
    patient_id      integer                                              not null
        constraint document_patients_id_fk
            references patients
            on delete set null,
    institution_id  integer
        constraint document_medical_institutions_id_fk
            references medical_institutions
            on delete set null,
    date_a          date,
    date_b          date,
    date_c          date,
    date_d          date,
    diagnosis       text,
    anamnesis       text,
    recommendations text,
    document_date   date
);

alter table documents
    owner to medical;




