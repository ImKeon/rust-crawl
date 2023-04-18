-- Your SQL goes here
CREATE TABLE league_rank
(
    id bigint auto_increment primary key,
    active boolean not null default true,
    created_at bigint null,
    created_id bigint null,
    updated_at bigint null,
    updated_id bigint null,
    issued_date bigint null,
    subject_rank int null,
    rank_title VARCHAR(64) null,
    subject_name VARCHAR(64) null
)