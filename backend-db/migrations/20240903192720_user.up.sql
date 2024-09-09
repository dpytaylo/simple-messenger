-- Add up migration script here
CREATE TABLE "user" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    registration_kind TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    "password" TEXT,
    "name" TEXT NOT NULL UNIQUE,
    avatar TEXT
);