#!/bin/bash
set -euo pipefail

# Run the simplified build script to ensure WASM binaries are up-to-date
./build.sh

# The list of all test cases (which correspond to the binary names)
TEST_CASES=(
  hex2i8
  hex2u8
  hex2i16be
  hex2i16le
  hex2u16be
  hex2u16le
  hex2i32be
  hex2i32le
  hex2u32be
  hex2u32le
  hex2i64be
  hex2i64le
  hex2u64be
  hex2u64le
)

# Directory containing the WASM files, as output by the build script
WASM_DIR="target/wasm32-wasip1/release-wasi"
# Directory containing the example JSON files
EXAMPLES_DIR="examples/json"

# Flag to track if any test fails
any_failed=0

# Loop through all test cases
for test_case in "${TEST_CASES[@]}"; do
  echo "--- Running test for ${test_case} ---"

  # Define file paths
  wasm_file="${WASM_DIR}/${test_case}.wasm"
  input_file="${EXAMPLES_DIR}/${test_case}-input.json"
  expected_output_file="${EXAMPLES_DIR}/${test_case}-output.json"
  
  # Temporary files for actual and standardized outputs
  actual_output_file=$(mktemp)
  pretty_expected_output_file=$(mktemp)
  pretty_actual_output_file=$(mktemp)

  # Check if all required files exist
  if ! [ -f "${wasm_file}" ]; then
    echo "[FAIL]: WASM file not found at ${wasm_file}"
    any_failed=1
    continue
  fi
  if ! [ -f "${input_file}" ]; then
    echo "[FAIL]: Input JSON not found at ${input_file}"
    any_failed=1
    continue
  fi
  if ! [ -f "${expected_output_file}" ]; then
    echo "[FAIL]: Expected output JSON not found at ${expected_output_file}"
    any_failed=1
    continue
  fi

  # Run the test using wazero and capture the output
  cat "${input_file}" | wazero run "${wasm_file}" > "${actual_output_file}"

  # Standardize JSON formatting for comparison using jq
  jq -S . "${expected_output_file}" > "${pretty_expected_output_file}"
  jq -S . "${actual_output_file}" > "${pretty_actual_output_file}"

  # Compare the standardized actual output with the standardized expected output
  if diff -u "${pretty_expected_output_file}" "${pretty_actual_output_file}"; then
    echo "[OK]: ${test_case}"
  else
    echo "[FAIL]: ${test_case} - Output mismatch."
    any_failed=1
  fi
  
  # Clean up temporary files
  rm "${actual_output_file}" "${pretty_expected_output_file}" "${pretty_actual_output_file}"
done

# Exit with a non-zero status code if any test failed
if [ "${any_failed}" -ne 0 ]; then
  echo ""
  echo "Some tests failed."
  exit 1
else
  echo ""
  echo "All tests passed successfully!"
fi