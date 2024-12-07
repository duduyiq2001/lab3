#!/bin/bash

# Path to the executable
EXECUTABLE="./target/release/lab_3"

SERVER_ADDRESS="127.0.0.1:1024"

# Check if the executable exists; if not, build it
if [ ! -f "$EXECUTABLE" ]; then
    echo "Executable not found at $EXECUTABLE. Building the project..."
    cargo build --release
fi

# Define test case details in a list
TEST_CASE_DETAILS=(
    "Test case 0: Test 'partial_hamlet_act_ii_script.txt' from server, check against 'baseline_ham.out'."
    "Test case 1: Multiple clients connect to the server."
    "Test case 2: Testing File doesn't exist on server."
)

# Function to display all test case details
display_test_case_details() {
    for i in "${!TEST_CASE_DETAILS[@]}"; do
        echo "  $i - ${TEST_CASE_DETAILS[$i]}"
    done
}

# Function to run a specific test case
run_test_case() {
    local TEST_ID="$1"
    echo "${TEST_CASE_DETAILS[$TEST_ID]}"
    case "$TEST_ID" in
    0)
        INPUT_FILE="net:$SERVER_ADDRESS:partial_hamlet_act_ii_script.txt"
        OUTPUT_FILE="test_ham.out"
        BASELINE_FILE="baseline_ham.out"

        echo "Command 1: Running test from server."
        $EXECUTABLE "$INPUT_FILE" >"$OUTPUT_FILE" 2>&1
        echo "Output are saved in '$OUTPUT_FILE'."

        echo "Command 2: Diffing output with '$BASELINE_FILE'. (ignoring empty lines and spaces)."
        DIFF_OUTPUT=$(diff -B -w -y "$OUTPUT_FILE" "$BASELINE_FILE")
        DIFF_EXIT_CODE=$? # get diff exit code

        if [ $DIFF_EXIT_CODE -eq 0 ]; then
            echo -e "✅ No differences found between '$OUTPUT_FILE' and '$BASELINE_FILE'."
        else
            echo -e "❌ Differences found:"
            echo "$DIFF_OUTPUT"
        fi
        ;;
    1)
        echo "Command 1: Multiple clients connect to the server."

        # Run each client command concurrently, redirecting output and errors
        "$EXECUTABLE" "net:$SERVER_ADDRESS:partial_hamlet_act_ii_script.txt" >multi1.txt 2>&1 &
        pid1=$!
        "$EXECUTABLE" "net:$SERVER_ADDRESS:partial_macbeth_act_i_script.txt" >multi2.txt 2>&1 &
        pid2=$!
        "$EXECUTABLE" "net:$SERVER_ADDRESS:partial_hamlet_act_ii_script.txt" >multi3.txt 2>&1 &
        pid3=$!
        "$EXECUTABLE" "net:$SERVER_ADDRESS:partial_macbeth_act_i_script.txt" >multi4.txt 2>&1 &
        pid4=$!

        # Wait for all background processes to finish
        wait $pid1 $pid2 $pid3 $pid4

        echo "All clients have connected to the server."
        ;;
    2)
        echo "Command 1: Testing File doesn't exist on server."
        $EXECUTABLE "net:$SERVER_ADDRESS:not_exist.txt"

        ;;
    esac
}

# Function to display usage information
display_usage() {
    echo "Usage: $0 [test_case_number]"
    echo ""
    echo "Available test cases:"
    display_test_case_details
}

# Check for parameters
if [ "$#" -eq 0 ]; then
    echo "No test case specified."
    display_usage
elif [ "$#" -eq 1 ]; then
    if [[ "$1" =~ ^[0-9]$ ]]; then
        run_test_case "$1"
        echo "Completed test case $1."
    else
        echo "Invalid test case number: $1"
        display_usage
        exit 1
    fi
else
    echo "Invalid number of arguments."
    display_usage
    exit 1
fi
