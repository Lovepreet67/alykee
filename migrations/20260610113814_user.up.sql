-- Create enums
CREATE TYPE user_role AS ENUM ('staff', 'admin');

CREATE TYPE task_status AS ENUM (
    'todo',
    'in_progress',
    'completed',
    'cancelled'
);

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    role user_role NOT NULL DEFAULT 'staff',
    hashed_password VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Task table
CREATE TABLE task (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description VARCHAR(1023) NOT NULL,

    status task_status NOT NULL DEFAULT 'todo',
    priority BIGINT NOT NULL,

    created_by_id UUID NOT NULL,
    assigned_to_id UUID NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT fk_task_created_by
        FOREIGN KEY (created_by_id)
        REFERENCES users(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE,

    CONSTRAINT fk_task_assigned_to
        FOREIGN KEY (assigned_to_id)
        REFERENCES users(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE
);