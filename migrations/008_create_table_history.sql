create type state as enum ('rejected', 'paid', 'claimed', 'completed');

CREATE TABLE order_history (
    history_id uuid PRIMARY KEY default gen_random_uuid(),
    order_status state not null,
    created_at TimeStamptz default now(),
    paid_at TimeStamptz,
    claimed_at TimeStamptz,
    completed_at TimeStamptz,

    order_of uuid not null references orders(order_id)
);
