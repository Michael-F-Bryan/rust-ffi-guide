typedef int (*callback)(int intermediate_result);

// Calculate the factorial of `start`, calling the callback after every
// iteration to notify the user of the current progress and check whether we
// should still continue.
void expensive_calculation(int start, callback cb) {
    int result = 1;

    for (int i = 1; i <= start; ++i) {
        result *= i;

        int should_continue = cb(result);
        if (!should_continue) {
            return;
        }
    }

    return;
}


typedef int (*stateful_callback)(void *state, int intermediate_result);


void stateful_expensive_calculation(int start, stateful_callback cb, void *state) {
    int result = 1;

    for (int i = 1; i <= start; ++i) {
        result *= i;

        int should_continue = cb(state, result);
        if (!should_continue) {
            return;
        }
    }

    return;
}

