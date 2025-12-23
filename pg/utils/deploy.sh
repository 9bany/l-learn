#!/bin/bash

# Script to deploy/uninstall PostgreSQL extension to Docker container
# Usage: 
#   ./deploy.sh [container_name]           - Install/update extension
#   ./deploy.sh --uninstall [container_name] - Uninstall extension

# Parse arguments
if [[ "$1" == "--uninstall" ]]; then
    UNINSTALL=true
    CONTAINER_NAME=${2:-etl-test}
else
    UNINSTALL=false
    CONTAINER_NAME=${1:-etl-test}
fi

EXT_NAME="addme"

if [[ "$UNINSTALL" == "true" ]]; then
    echo "Uninstalling $EXT_NAME extension from Docker container: $CONTAINER_NAME"
else
    echo "Deploying $EXT_NAME extension to Docker container: $CONTAINER_NAME"
fi

# Uninstall mode
if [[ "$UNINSTALL" == "true" ]]; then
    echo "Dropping extension from database..."
    PGPASSWORD=postgres psql -h localhost -p 5430 -U postgres -d postgres -c "DROP EXTENSION IF EXISTS $EXT_NAME CASCADE;"
    
    echo "Removing extension files from container..."
    docker exec $CONTAINER_NAME bash -c "
        rm -f /usr/share/postgresql/18/extension/$EXT_NAME.control
        rm -f /usr/share/postgresql/18/extension/$EXT_NAME--*.sql
        rm -f /usr/lib/postgresql/18/lib/$EXT_NAME.so
        rm -f /usr/lib/postgresql/18/lib/bitcode/$EXT_NAME.index.bc
        rm -rf /usr/lib/postgresql/18/lib/bitcode/$EXT_NAME/
    "
    
    echo "Uninstall completed!"
    exit 0
fi

# Install mode (default)
# 1. Copy source files to container
echo "Copying source files..."
docker cp $EXT_NAME.c $CONTAINER_NAME:/tmp/
docker cp $EXT_NAME.control $CONTAINER_NAME:/tmp/
docker cp $EXT_NAME--0.0.1.sql $CONTAINER_NAME:/tmp/
docker cp Makefile $CONTAINER_NAME:/tmp/

# 2. Compile and install in container
echo "🔨 Compiling and installing extension..."
docker exec $CONTAINER_NAME bash -c "cd /tmp && make clean && make && make install"

# 3. Check if extension exists and drop it
echo "Dropping existing extension (if exists)..."
PGPASSWORD=postgres psql -h localhost -p 5430 -U postgres -d postgres -c "DROP EXTENSION IF EXISTS $EXT_NAME CASCADE;" 2>/dev/null

# 4. Create extension
echo "Creating extension..."
PGPASSWORD=postgres psql -h localhost -p 5430 -U postgres -d postgres -c "CREATE EXTENSION $EXT_NAME;"

# 5. Test functions
echo "Testing extension..."
PGPASSWORD=postgres psql -h localhost -p 5430 -U postgres -d postgres -c "
SELECT addme(5, 3) as add_result;
SELECT multiply(4, 7) as multiply_result;
"

echo "Deploy completed!"
