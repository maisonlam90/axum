-- Add migration script here
-- migrations/0001_init_schema.sql

-- Tenant table
CREATE TABLE tenants (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL
);

-- Users table
CREATE TABLE users (
  id UUID PRIMARY KEY,
  tenant_id UUID REFERENCES tenants(id),
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT NOT NULL,
  is_active BOOLEAN DEFAULT TRUE
);

-- Groups table
CREATE TABLE groups (
  id UUID PRIMARY KEY,
  tenant_id UUID REFERENCES tenants(id),
  name TEXT NOT NULL
);

-- Permissions table
CREATE TABLE permissions (
  id UUID PRIMARY KEY,
  code TEXT UNIQUE NOT NULL -- e.g. 'user.create'
);

-- User-Group mapping
CREATE TABLE user_groups (
  user_id UUID REFERENCES users(id),
  group_id UUID REFERENCES groups(id),
  PRIMARY KEY (user_id, group_id)
);

-- Group-Permission mapping
CREATE TABLE group_permissions (
  group_id UUID REFERENCES groups(id),
  permission_id UUID REFERENCES permissions(id),
  PRIMARY KEY (group_id, permission_id)
);
