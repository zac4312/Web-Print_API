create type state as enum ('rejected', 'paid', 'claimed', 'completed');

CREATE TABLE order_history (
    history_id uuid PRIMARY KEY default gen_random_uuid(),
    order_status state not null,
    created_at TimeStampz default now(),
    paid_at TimeStampz,
    claimed_at TimeStampz,
    completed_at TimeStampz
)
