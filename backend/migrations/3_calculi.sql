
create table "calculi" (
    calculi_id      uuid primary key                            default gen_random_uuid(),

    win_length      int                             not null,

    board_size      int check (board_size <= 50)    not null,

    move_time_limit int                             not null,

    game_time_limit int                             not null,

    created_at      timestamptz                     not null    default now(),

    end_time        timestamptz,

    updated_at      timestamptz
);

SELECT trigger_updated_at('"calculi"');