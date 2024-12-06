#!/bin/bash

# Path to the executable
EXECUTABLE="./target/release/lab_3"

# Check if the executable exists; if not, build it
if [ ! -f "$EXECUTABLE" ]; then
    echo "Executable not found at $EXECUTABLE. Building the project..."
    cargo build --release
fi

# Define test case details in a list
TEST_CASE_DETAILS=(
    "Test case 0: Test 'partial_hamlet_act_ii_script.txt', check against 'baseline_ham.out'."
    "Test case 1: Test 'partial_hamlet_act_ii_script.txt'. with argument 'whinge'."
    "Test case 2: Test 'partial_macbeth_act_i_script.txt'. check against 'baseline_mac.out'."
    "Test case 3: Test 'partial_macbeth_act_i_script.txt'. with argument 'whinge'."
    "Test case 4: Test 'partial_macbeth_act_i_script_mod.txt'. check against 'baseline_mac_mod.out'."
    "Test case 5: Test 'partial_macbeth_act_i_script_mod.txt'. with argument 'whinge'."
    "Test case 6: Test 'partial_mac_script_mod2.txt'."
    "Test case 7: Test 'partial_mac_script_mod3.txt'."
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
        INPUT_FILE="partial_hamlet_act_ii_script.txt"
        OUTPUT_FILE="test_ham.out"
        BASELINE_FILE="baseline_ham.out"

        echo "Command 1: Running test."
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
        INPUT_FILE="partial_hamlet_act_ii_script.txt"
        OUTPUT_FILE="test_ham_whinge.out"
        BASELINE_FILE="baseline_ham.out"

        echo "Command 1: Running test with argument 'whinge'."
        $EXECUTABLE "$INPUT_FILE" whinge >"$OUTPUT_FILE" 2>&1
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
    2)
        INPUT_FILE="partial_macbeth_act_i_script.txt"
        OUTPUT_FILE="test_mac.out"
        BASELINE_FILE="baseline_mac.out"

        echo "Command 1: Running test."
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
    3)
        INPUT_FILE="partial_macbeth_act_i_script.txt"
        OUTPUT_FILE="test_mac_whinge.out"
        BASELINE_FILE="baseline_mac.out"

        echo "Command 1: Running test with argument 'whinge'."
        $EXECUTABLE "$INPUT_FILE" whinge >"$OUTPUT_FILE" 2>&1
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
    4)
        INPUT_FILE="partial_macbeth_act_i_script_mod.txt"
        OUTPUT_FILE="test_mac_mod.out"
        BASELINE_FILE="baseline_mac_mod.out"

        echo "Command 1: Running test."
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
    5)
        INPUT_FILE="partial_macbeth_act_i_script_mod.txt"
        OUTPUT_FILE="test_mac_mod_whinge.out"
        BASELINE_FILE="baseline_mac_mod.out"

        echo "Command 1: Running test with argument 'whinge'."
        $EXECUTABLE "$INPUT_FILE" whinge >"$OUTPUT_FILE" 2>&1
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
    6)
        INPUT_FILE="partial_mac_script_mod2.txt"
        OUTPUT_FILE="test_mac_mod2.out"
        echo "Command 1: Running test."
        echo "===============OUTPUT========================"
        $EXECUTABLE "$INPUT_FILE" 2>&1 | tee "$OUTPUT_FILE"
        echo "===============END OF OUTPUT========================"
        echo "Output are saved in '$OUTPUT_FILE'."
        ;;
    7)
        INPUT_FILE="partial_mac_script_mod3.txt"
        OUTPUT_FILE="test_mac_mod3.out"
        echo "Command 1: Running test."
        echo "===============OUTPUT========================"
        $EXECUTABLE "$INPUT_FILE" 2>&1 | tee "$OUTPUT_FILE"
        echo "===============END OF OUTPUT========================"
        echo "Output are saved in '$OUTPUT_FILE'."
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
    if [[ "$1" =~ ^[0-7]$ ]]; then
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
