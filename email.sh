#!/bin/bash

# Function to generate a random email address
generate_random_email() {
    local length=$((RANDOM % 11 + 5)) # Random length between 5 and 15 characters
    local email=$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c $length ; echo '')
    echo "${email}@example.com" # Appending a domain to make it a valid email
}

# Generate 100,000 random email addresses
emails=()
for ((i = 0; i < 100000; i++)); do
    emails+=("$(generate_random_email)")
done

# Convert the array of emails to JSON format
emails_json=$(printf '%s\n' "${emails[@]}" | jq -R . | jq -s .)

# Write the JSON to a file
echo "$emails_json" > emails.json

echo "JSON file containing 100,000 random email addresses generated."

