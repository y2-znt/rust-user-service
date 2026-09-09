-- Local seed data. Re-runnable.
--   psql "$DATABASE_URL" -f seeds/users.sql

INSERT INTO users (id, email, username, password_hash, role)
VALUES
    (
        '550e8400-e29b-41d4-a716-446655440000',
        'y2@example.com',
        'y2',
        'fake_hash_for_now',
        'user'
    ),
    (
        '550e8400-e29b-41d4-a716-446655440001',
        'ada@example.com',
        'ada',
        'fake_hash_for_now',
        'admin'
    ),
    (
        '550e8400-e29b-41d4-a716-446655440002',
        'grace@example.com',
        'grace',
        'fake_hash_for_now',
        'user'
    ),
    (
        '550e8400-e29b-41d4-a716-446655440003',
        'linus@example.com',
        'linus',
        'fake_hash_for_now',
        'user'
    ),
    (
        '550e8400-e29b-41d4-a716-446655440004',
        'alice@example.com',
        'alice',
        'fake_hash_for_now',
        'user'
    )
ON CONFLICT (id) DO NOTHING;
