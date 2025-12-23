MODULES = addme
EXTENSION = addme
DATA = addme--0.0.1.sql

# Database connection parameters
DB_HOST = localhost
DB_PORT = 5430
DB_USER = postgres
DB_PASS = postgres
DB_NAME = postgres

PG_CONFIG = pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)
