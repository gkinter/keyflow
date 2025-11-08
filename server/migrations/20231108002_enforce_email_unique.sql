CREATE UNIQUE INDEX IF NOT EXISTS users_email_unique
ON users (LOWER(email))
WHERE email IS NOT NULL;

