/*
 * dbinit.sql: raw SQL to initialize a database for use by Lumen
 */

CREATE DATABASE lumen;
CREATE USER lumen;
ALTER DEFAULT PRIVILEGES GRANT INSERT, SELECT, UPDATE, DELETE ON TABLES TO lumen;
ALTER USER lumen PASSWORD 'lumen';
