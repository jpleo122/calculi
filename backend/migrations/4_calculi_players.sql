create table calculi_participant (
    calculi_participant_id  uuid primary key                                    default gen_random_uuid(),
    
    calculi_game_id         uuid references calculi(calculi_id)     not null,
    
    user_id                 uuid references "user"(user_id)         not null,
    
    board                   bit(2500)                               not null,
    
    score                   decimal(5, 2)
);