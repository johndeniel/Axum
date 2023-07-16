-- Create the 'users' table to store user information
CREATE TABLE IF NOT EXISTS users (
  id          SERIAL PRIMARY KEY,            -- Auto-incrementing unique ID for each user
  username    VARCHAR(64) NOT NULL UNIQUE,   -- Username, must be unique and not null
  password    VARCHAR(64) NOT NULL,          -- Password for the user, not null
  deleted_at  TIMESTAMPTZ DEFAULT NULL,      -- Timestamp for soft deletion (when user is deleted)
  token       TEXT DEFAULT NULL              -- Token field (not used in this script)
);

-- Create the 'tasks' table to store task information
CREATE TABLE IF NOT EXISTS tasks (
  id            SERIAL PRIMARY KEY,          -- Auto-incrementing unique ID for each task
  priority      VARCHAR(4) DEFAULT NULL,     -- Task priority (can be null)
  title         VARCHAR(255) NOT NULL,       -- Title of the task, must not be null
  completed_at  TIMESTAMPTZ DEFAULT NULL,    -- Timestamp for task completion (when task is completed)
  description   TEXT DEFAULT NULL,           -- Description of the task (can be null)
  deleted_at    TIMESTAMPTZ DEFAULT NULL,    -- Timestamp for soft deletion (when task is deleted)
  user_id       INTEGER DEFAULT NULL,        -- Foreign key referencing user ID associated with the task
  is_default    BOOLEAN DEFAULT FALSE,       -- Boolean field indicating if the task is a default task
  CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id) -- Foreign key constraint for 'user_id'
);

-- Insert a sample user with username 'deleteduser' and hashed password
INSERT INTO users (username, password) VALUES ('deleteduser', '$2b$12$x3hs5oMgjHdcV1GUEElfsO19JtS6.ixJAX9Cj62GyhpdPAIW25sky');

-- Insert a sample task associated with the 'deleteduser' user
INSERT INTO tasks (title, deleted_at, user_id) VALUES (
  'my deleted task',                         -- Title of the task
  NOW(),                                     -- Current timestamp for soft deletion
  (SELECT id FROM users WHERE username = 'deleteduser') -- User ID of the 'deleteduser'
);

-- Insert two sample tasks with different priorities and a default status
INSERT INTO tasks (priority, title, description, is_default) VALUES 
  ('A', 'I am a task, you can complete me by checking the box', 'This is my description', true),
  ('B', 'See my details for by clicking me', 'My description can be changed', true);