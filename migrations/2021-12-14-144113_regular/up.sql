-- Your SQL goes here
alter table users add date_pass_changed date not null default current_date;