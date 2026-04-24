
create table "calculi_move" (
    calculi_move_id         uuid primary key                                                            default gen_random_uuid(),

    calculi_id              uuid references calculi(calculi_id)                             not null,

    participant_id          uuid references calculi_participant(calculi_participant_id)     not null,

    move_position           point                                                           not null,

    created_at              timestamptz                                                     not null    default now(),

    updated_at              timestamptz
);

SELECT trigger_updated_at('"calculi_move"');