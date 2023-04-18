-- Your SQL goes here
CREATE TABLE schedule
(
    id bigint auto_increment primary key,
    active boolean not null default true,
    created_at bigint null,
    created_id bigint null,
    updated_at bigint null,
    updated_id bigint null,
    game_schedule_day bigint null,
    home_team VARCHAR(64) null,
    away_team VARCHAR(64) null,
    stadium VARCHAR(64) null
)
