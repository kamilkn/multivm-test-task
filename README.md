# VM test task

## Overview

This project demonstrates a multithreaded computation in Rust and calculates the Collatz problem steps. It is divided into two main parts:

1. A function that distributes computation work across multiple threads only when the input size exceeds a specified threshold.
2. A function that solves the Collatz problem for each input number with a given iteration limit.

## Collatz Problem

For any natural number `n`:

- If `n` is even, the next number will be `n/2`.
- If `n` is odd, the next number will be `3 * n + 1`.
- The iteration ends when `n` reaches 1 or exceeds a predefined iteration limit.

## Functions

### `transform_number(number: u64, max_steps: usize) -> u64`

Calculates the number of steps required to reduce the given number to 1 or returns the final value if the maximum step count is exceeded.

**Parameters:**

- `number` - Starting number.
- `max_steps` - Maximum number of steps allowed.

**Returns:**

- Number of steps if the limit was not exceeded, otherwise the final value.

## Usage

1. **Set Up Environment Variables**

    Create or modify a `.env` file in the root of the project to configure the computation parameters:

    ```bash
    echo -e "THRESHOLD=3\nMAX_ITERATIONS=8" > .env
    ```

    - `THRESHOLD`: The minimum input size required to parallelize the computation. Default is `3`.
    - `MAX_ITERATIONS`: The maximum number of iterations before stopping the computation. Default is `8`.

2. **Build and Run the Project**

    To execute the application with sample data:

    ```bash
    cargo run -- 1 2 3 100
    ```

## Testing

```bash
cargo nextest run
