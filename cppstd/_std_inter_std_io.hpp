#include <iostream>


void _std_inter_print(const std::string& str) {
    std::cout << str;
}

std::string _std_inter_input() {
    std::string in{""};

    std::cin >> in;

    return in;
}