// @generated automatically by Diesel CLI.

diesel::table! {
    league_rank (id) {
        id -> Bigint,
        active -> Bool,
        created_at -> Nullable<Bigint>,
        created_id -> Nullable<Bigint>,
        updated_at -> Nullable<Bigint>,
        updated_id -> Nullable<Bigint>,
        issued_date -> Nullable<Bigint>,
        subject_rank -> Nullable<Integer>,
        rank_title -> Nullable<Varchar>,
        subject_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    schedule (id) {
        id -> Bigint,
        active -> Bool,
        created_at -> Nullable<Bigint>,
        created_id -> Nullable<Bigint>,
        updated_at -> Nullable<Bigint>,
        updated_id -> Nullable<Bigint>,
        game_schedule_day -> Nullable<Bigint>,
        home_team -> Nullable<Varchar>,
        away_team -> Nullable<Varchar>,
        stadium -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    league_rank,
    schedule,
);
