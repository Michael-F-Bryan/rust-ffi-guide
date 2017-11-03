#include <iostream>

extern "C" {
    char *home_directory();
}

int main() {
    char* home = home_directory();

    if (home == nullptr) {
        std::cout << "Unable to find the home directory" << std::endl;
    } else {
        std::cout << "Home directory is " << home << std::endl; 
    }
}
