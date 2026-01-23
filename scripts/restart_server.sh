#!/bin/bash

PORT=3000

# Check if lsof is installed
if ! command -v lsof &> /dev/null; then
    echo "Error: lsof is not installed. Please install it to use this script."
    exit 1
fi

# Find PID listening on the port
PID=$(lsof -ti:$PORT)

if [ -n "$PID" ]; then
  echo "Killing process on port $PORT (PID: $PID)..."
  kill -9 $PID
  # Wait a moment to ensure port is freed
  sleep 1
else
  echo "No process found on port $PORT."
fi

echo "Starting server..."

cargo run
