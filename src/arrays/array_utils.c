int sum(const int* my_array, int length) {
    int total = 0;

    for(int i = 0; i < length; i++) {
        total += my_array[i];
    }
    
    return total;
}