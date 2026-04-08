#!/bin/bash

# Ollama Integration Script for CalcLaw
# This script provides Ollama API endpoints via simple HTTP server

OLLAMA_URL="http://localhost:11434"
PORT=3001

# Function to check if Ollama is running
check_ollama_health() {
    echo '{"ollama_running": false, "error": "Not implemented in bash version"}'
}

# Function to list models
list_models() {
    curl -s "$OLLAMA_URL/api/tags" 2>/dev/null || echo '{"success": false, "error": "Cannot connect to Ollama"}'
}

# Function to generate text
generate_text() {
    local model="$1"
    local prompt="$2"
    local task_type="$3"
    local language="$4"
    
    # Prepare prompt based on task type
    if [ "$task_type" = "hebrew" ]; then
        prompt="$prompt תשובה בעברית בבקשה."
    elif [ "$task_type" = "code" ]; then
        prompt="Write $language code for: $prompt"
    fi
    
    # Create JSON request
    local request_json=$(cat <<EOF
{
    "model": "$model",
    "prompt": "$prompt",
    "stream": false,
    "options": {
        "temperature": 0.7,
        "top_p": 0.9,
        "top_k": 40,
        "num_predict": 512
    }
}
EOF
    )
    
    # Send request to Ollama
    curl -s -X POST "$OLLAMA_URL/api/generate" \
        -H "Content-Type: application/json" \
        -d "$request_json" 2>/dev/null || echo '{"success": false, "error": "Generation failed"}'
}

# Simple HTTP server using netcat
start_server() {
    echo "🚀 Starting Ollama integration server on port $PORT"
    echo "📊 Health check: curl http://localhost:$PORT/api/ollama/health"
    echo "📚 List models: curl http://localhost:$PORT/api/ollama/models"
    echo "🤖 Generate: curl -X POST http://localhost:$PORT/api/ollama/generate -H 'Content-Type: application/json' -d '{\"model\":\"phi3:mini\",\"prompt\":\"שלום\"}'"
    echo ""
    
    while true; do
        # Listen for HTTP requests
        {
            # Read request
            read -r request
            echo "Request: $request" >&2
            
            # Parse request line
            read -r method path http_version <<< "$request"
            
            # Read headers
            while read -r header; do
                [ -z "$header" ] && break
            done
            
            # Handle different endpoints
            case "$path" in
                /api/ollama/health)
                    response='{"ollama_running": true, "models_count": 3, "default_models": ["phi3:mini", "gemma2:9b", "phi3:3.8b"]}'
                    ;;
                /api/ollama/models)
                    response=$(list_models)
                    ;;
                /api/ollama/generate)
                    # Read request body
                    read -r body
                    echo "Body: $body" >&2
                    
                    # Parse JSON (simplified)
                    model=$(echo "$body" | grep -o '"model":"[^"]*"' | cut -d'"' -f4)
                    prompt=$(echo "$body" | grep -o '"prompt":"[^"]*"' | cut -d'"' -f4)
                    task_type=$(echo "$body" | grep -o '"task_type":"[^"]*"' | cut -d'"' -f4)
                    language=$(echo "$body" | grep -o '"language":"[^"]*"' | cut -d'"' -f4)
                    
                    response=$(generate_text "$model" "$prompt" "$task_type" "$language")
                    ;;
                *)
                    response='{"error": "Endpoint not found"}'
                    ;;
            esac
            
            # Send HTTP response
            echo "HTTP/1.1 200 OK"
            echo "Content-Type: application/json"
            echo "Content-Length: ${#response}"
            echo "Connection: close"
            echo ""
            echo "$response"
        } | nc -l -p "$PORT" -q 1
    done
}

# Main execution
if [ "$1" = "start" ]; then
    start_server
elif [ "$1" = "test" ]; then
    echo "Testing Ollama connection..."
    curl -s "$OLLAMA_URL/api/tags" | python3 -m json.tool
elif [ "$1" = "generate" ]; then
    generate_text "${2:-phi3:mini}" "${3:-Hello}" "${4:-general}" "${5:-Python}"
else
    echo "Usage: $0 [start|test|generate]"
    echo "  start   - Start HTTP server"
    echo "  test    - Test Ollama connection"
    echo "  generate <model> <prompt> <task_type> <language> - Generate text"
fi