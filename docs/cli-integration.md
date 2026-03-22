# AIDEN CLI Integration Guide

Command-line tools and scripts for working with AIDEN.

## Table of Contents

1. [Basic CLI Tools](#basic-cli-tools)
2. [Interactive Chat](#interactive-chat)
3. [Batch Processing](#batch-processing)
4. [Script Examples](#script-examples)
5. [Shell Aliases](#shell-aliases)
6. [Automation Scripts](#automation-scripts)

---

## Basic CLI Tools

### Health Check

```bash
#!/bin/bash
# health-check.sh

HEALTH=$(curl -s http://localhost:8081/api/health)
STATUS=$(echo $HEALTH | jq -r '.status')
OLLAMA=$(echo $HEALTH | jq -r '.ollama')
QDRANT=$(echo $HEALTH | jq -r '.qdrant')

echo "AIDEN Health Check"
echo "=================="
echo "Overall Status: $STATUS"
echo "Ollama: $OLLAMA"
echo "Qdrant: $QDRANT"

if [ "$STATUS" = "healthy" ]; then
    exit 0
else
    exit 1
fi
```

### Quick Chat

```bash
#!/bin/bash
# aiden-chat.sh

MESSAGE="${1:-Hello, how are you?}"

curl -s -X POST http://localhost:8081/api/chat \
  -H "Content-Type: application/json" \
  -d "{\"message\": \"$MESSAGE\"}" \
  | jq -r '.response.content'
```

Usage:
```bash
./aiden-chat.sh "What is AcreetionOS?"
```

---

## Interactive Chat

### Bash Interactive Client

```bash
#!/bin/bash
# aiden-interactive.sh

echo "AIDEN Interactive Chat"
echo "Type 'exit' to quit"
echo "====================="
echo ""

while true; do
    read -p "You: " message
    
    if [ "$message" = "exit" ]; then
        echo "Goodbye!"
        break
    fi
    
    response=$(curl -s -X POST http://localhost:8081/api/chat \
        -H "Content-Type: application/json" \
        -d "{\"message\": \"$message\"}" \
        | jq -r '.response.content')
    
    echo "AIDEN: $response"
    echo ""
done
```

### Python Interactive Client

```python
#!/usr/bin/env python3
# aidenInteractive.py

import requests
import sys

def chat(message):
    response = requests.post(
        "http://localhost:8081/api/chat",
        json={"message": message},
        timeout=120
    )
    return response.json()["response"]["content"]

def main():
    print("AIDEN Interactive Chat")
    print("Type 'exit' to quit\n")
    
    while True:
        try:
            message = input("You: ").strip()
            
            if message.lower() in ['exit', 'quit', 'q']:
                print("Goodbye!")
                break
            
            if not message:
                continue
            
            response = chat(message)
            print(f"AIDEN: {response}\n")
            
        except KeyboardInterrupt:
            print("\nGoodbye!")
            break
        except Exception as e:
            print(f"Error: {e}\n")

if __name__ == "__main__":
    main()
```

---

## Batch Processing

### Process Questions from File

```bash
#!/bin/bash
# batch-chat.sh

INPUT_FILE="${1:-questions.txt}"
OUTPUT_FILE="${2:-answers.txt}"

echo "Processing questions from $INPUT_FILE..."
echo ""

while IFS= read -r question; do
    if [ -z "$question" ] || [[ "$question" =~ ^# ]]; then
        continue
    fi
    
    echo "Q: $question"
    
    answer=$(curl -s -X POST http://localhost:8081/api/chat \
        -H "Content-Type: application/json" \
        -d "{\"message\": \"$question\"}" \
        | jq -r '.response.content')
    
    echo "A: $answer"
    echo "---"
    
    echo "$question | $answer" >> "$OUTPUT_FILE"
    
    sleep 1  # Rate limiting
    
done < "$INPUT_FILE"

echo ""
echo "Done! Answers saved to $OUTPUT_FILE"
```

### Python Batch Processor

```python
#!/usr/bin/env python3
# batchProcess.py

import requests
import json
import time
from pathlib import Path

def chat(message, retries=3):
    for i in range(retries):
        try:
            response = requests.post(
                "http://localhost:8081/api/chat",
                json={"message": message},
                timeout=120
            )
            return response.json()["response"]["content"]
        except Exception as e:
            if i < retries - 1:
                time.sleep(5)
            else:
                return f"ERROR: {e}"

def process_file(input_file, output_file):
    input_path = Path(input_file)
    output_path = Path(output_file)
    
    print(f"Processing {input_path}...")
    
    with open(input_path) as f:
        questions = f.readlines()
    
    results = []
    for i, question in enumerate(questions, 1):
        question = question.strip()
        if not question or question.startswith('#'):
            continue
        
        print(f"[{i}/{len(questions)}] {question[:50]}...")
        
        answer = chat(question)
        results.append({
            "question": question,
            "answer": answer,
            "timestamp": time.time()
        })
        
        time.sleep(0.5)  # Rate limiting
    
    with open(output_path, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"Done! Results saved to {output_path}")

if __name__ == "__main__":
    import sys
    input_file = sys.argv[1] if len(sys.argv) > 1 else "questions.txt"
    output_file = sys.argv[2] if len(sys.argv) > 2 else "answers.json"
    process_file(input_file, output_file)
```

### questions.txt Example
```
How do I install AcreetionOS?
What are the system requirements?
How do I configure the firewall?
How do I update packages?
What desktop environments are available?
```

---

## Script Examples

### Check and Restart Services

```bash
#!/bin/bash
# check-services.sh

check_service() {
    local service=$1
    local port=$2
    
    if systemctl is-active --quiet "$service"; then
        echo "[OK] $service is running"
        return 0
    else
        echo "[FAIL] $service is not running"
        return 1
    fi
}

restart_all() {
    echo "Restarting all services..."
    
    sudo systemctl restart ollama
    sleep 2
    
    sudo systemctl restart qdrant
    sleep 2
    
    sudo systemctl restart aiden
    sleep 2
    
    echo "Done!"
}

echo "Checking AIDEN services..."
echo "========================"

check_service "ollama" || restart_all
check_service "qdrant" || restart_all
check_service "aiden" || restart_all

echo ""
echo "Checking API..."
curl -s http://localhost:8081/api/health | jq '.'
```

### Index Documentation

```bash
#!/bin/bash
# index-docs.sh

echo "Indexing documentation..."
echo ""

response=$(curl -s -X POST http://localhost:8081/api/index)
echo "Status: $(echo $response | jq -r '.status')"

echo ""
echo "Waiting for indexing to complete..."
echo ""

while true; do
    status=$(curl -s http://localhost:8081/api/index/status)
    is_indexing=$(echo $status | jq -r '.is_indexing')
    files=$(echo $status | jq -r '.files_processed')
    chunks=$(echo $status | jq -r '.chunks_created')
    
    echo -ne "\rFiles: $files | Chunks: $chunks | Indexing: $is_indexing   "
    
    if [ "$is_indexing" = "false" ]; then
        echo ""
        echo ""
        if [ ${#errors[@]} -eq 0 ]; then
            echo "Indexing complete!"
        else
            echo "Indexing complete with errors"
            echo $status | jq '.errors'
        fi
        break
    fi
    
    sleep 2
done
```

### Generate FAQ

```bash
#!/bin/bash
# generate-faq.sh

OUTPUT_FILE="faq-generated.md"

cat > "$OUTPUT_FILE" << 'EOF'
# Frequently Asked Questions

Generated from AIDEN knowledge base.

---

EOF

questions=(
    "What is AcreetionOS?"
    "How do I install AcreetionOS?"
    "How do I update the system?"
    "What are the system requirements?"
    "How do I configure networking?"
    "How do I install software?"
    "How do I use the package manager?"
    "What desktop environments are available?"
    "How do I set up users?"
    "How do I configure the firewall?"
)

for question in "${questions[@]}"; do
    echo "Processing: $question"
    
    answer=$(curl -s -X POST http://localhost:8081/api/chat \
        -H "Content-Type: application/json" \
        -d "{\"message\": \"$question\"}" \
        | jq -r '.response.content')
    
    cat >> "$OUTPUT_FILE" << ANSWER
## $question

$answer

---
ANSWER
    
    sleep 1
done

echo ""
echo "FAQ generated: $OUTPUT_FILE"
```

---

## Shell Aliases

Add to `~/.bashrc` or `~/.zshrc`:

```bash
# AIDEN aliases
alias aiden-health='curl -s http://localhost:8081/api/health | jq "."'
alias aiden-status='curl -s http://localhost:8081/api/index/status | jq "."'
alias aiden-index='curl -s -X POST http://localhost:8081/api/index'
alias aiden-chat='curl -s -X POST http://localhost:8081/api/chat -H "Content-Type: application/json" -d "{\"message\": \"$1\"}" | jq -r ".response.content"'

# AIDEN service management
alias aiden-start='sudo systemctl start aiden ollama qdrant'
alias aiden-stop='sudo systemctl stop aiden'
alias aiden-restart='sudo systemctl restart aiden'
alias aiden-logs='journalctl -u aiden -f'

# Quick questions
alias ask-aiden='aiden-chat'
```

Reload shell:
```bash
source ~/.bashrc
```

---

## Automation Scripts

### Daily Digest

```bash
#!/bin/bash
# daily-digest.sh

DATE=$(date +%Y-%m-%d)
OUTPUT_DIR="/var/www/aiden/digest"
mkdir -p "$OUTPUT_DIR"

cat > "$OUTPUT_DIR/$DATE.md" << EOF
# AIDEN Daily Digest
## $(date)

### Quick Start Guide

EOF

topics=(
    "How do I get started with AcreetionOS?"
    "What are the first things to do after installation?"
    "How do I keep my system secure?"
)

for topic in "${topics[@]}"; do
    echo "Getting: $topic"
    
    content=$(curl -s -X POST http://localhost:8081/api/chat \
        -H "Content-Type: application/json" \
        -d "{\"message\": \"$topic\"}" \
        | jq -r '.response.content')
    
    cat >> "$OUTPUT_DIR/$DATE.md" << CONTENT
#### $topic

$content

CONTORY
    
    sleep 1
done

echo "" >> "$OUTPUT_DIR/$DATE.md"
echo "---" >> "$OUTPUT_DIR/$DATE.md"
echo "*Generated on $(date)*" >> "$OUTPUT_DIR/$DATE.md"

echo "Digest created: $OUTPUT_DIR/$DATE.md"
```

### Weekly Report

```bash
#!/bin/bash
# weekly-report.sh

# Add to crontab: 0 0 * * 1 /usr/local/bin/weekly-report.sh

DATE=$(date +%Y-W%V)
REPORT="/var/reports/aiden-weekly-$DATE.md"

mkdir -p "$(dirname $REPORT)"

{
    echo "# AIDEN Weekly Report"
    echo "## Week of $(date)"
    echo ""
    
    echo "### System Health"
    curl -s http://localhost:8081/api/health | jq '.'
    echo ""
    
    echo "### Index Statistics"
    curl -s http://localhost:8081/api/index/status | jq '.'
    echo ""
    
    echo "### Recent Questions Answered"
    
    # Sample questions for the week
    echo "1. How do I backup my system?" >> "$REPORT"
    echo "2. What is the best way to monitor resources?" >> "$REPORT"
    echo "3. How do I set up remote access?" >> "$REPORT"
    
} > "$REPORT"

echo "Report created: $REPORT"
```

### System Health Dashboard

```bash
#!/bin/bash
# dashboard.sh

clear

while true; do
    clear
    
    echo "╔════════════════════════════════════════════╗"
    echo "║         AIDEN System Dashboard             ║"
    echo "╚════════════════════════════════════════════╝"
    echo ""
    
    # Date
    echo "Date: $(date)"
    echo ""
    
    # Health
    HEALTH=$(curl -s http://localhost:8081/api/health)
    STATUS=$(echo $HEALTH | jq -r '.status')
    
    echo "┌────────────────────────────────────────────┐"
    echo "│ System Status: $STATUS" 
    printf "│ Ollama:        %s\n" "$(echo $HEALTH | jq -r '.ollama')"
    printf "│ Qdrant:        %s\n" "$(echo $HEALTH | jq -r '.qdrant')"
    echo "└────────────────────────────────────────────┘"
    echo ""
    
    # Index Status
    INDEX=$(curl -s http://localhost:8081/api/index/status)
    echo "┌────────────────────────────────────────────┐"
    printf "│ Index Status:  %s\n" "$(echo $INDEX | jq -r '.is_indexing')"
    printf "│ Files:         %s\n" "$(echo $INDEX | jq -r '.files_processed')"
    printf "│ Chunks:        %s\n" "$(echo $INDEX | jq -r '.chunks_created')"
    echo "└────────────────────────────────────────────┘"
    echo ""
    
    # GPU Status
    echo "┌────────────────────────────────────────────┐"
    nvidia-smi --query-gpu=utilization.gpu,memory.used,memory.total,temperature.gpu --format=csv,noheader,nounits | \
        while read line; do
            printf "│ GPU: %s%% | %sMiB / %sMiB | %s°C\n" $line
        done
    echo "└────────────────────────────────────────────┘"
    echo ""
    
    # Quick Actions
    echo "Press [Ctrl+C] to exit"
    
    sleep 5
done
```

---

## Cron Jobs

Add to crontab (`crontab -e`):

```cron
# Health check every 5 minutes
*/5 * * * * /usr/local/bin/health-check.sh

# Daily digest at 8 AM
0 8 * * * /usr/local/bin/daily-digest.sh

# Weekly report on Monday at 9 AM
0 9 * * 1 /usr/local/bin/weekly-report.sh

# Backup every day at 2 AM
0 2 * * * /usr/local/bin/backup-aiden.sh

# Index documentation every Sunday at 3 AM
0 3 * * 0 /usr/local/bin/index-docs.sh
```
