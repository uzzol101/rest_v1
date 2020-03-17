-- Your SQL goes here
CREATE TABLE users (
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  password VARCHAR(256) NOT NULL,
  company VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL
);
CREATE INDEX users_email_company_idx ON users (email, company);