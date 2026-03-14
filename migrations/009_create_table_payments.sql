create type payment_method as enum ('on-site', 'paypal');

CREATE TABLE payments (
   payment_id uuid PRIMARY KEY default gen_random_uuid(),
   transaction_id text not null,
   method payment_method not null,
   amount_paid DECIMAL(10,2),
   paid_at TimeStamptz
);
