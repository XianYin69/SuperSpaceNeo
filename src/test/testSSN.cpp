//
// Created by EthanYan on 2026/4/9.
//
#include <stdio.h>
#include <iostream>

//It is a menu to run testTool

class display_option {
protected:
    std::string option;
    std::string title;
public:
    display_option(std::string option, std::string title): option(option), title(title) {};
    void display() {
        std::cout << this -> option << ".\t" << this -> title << std::endl;
    }
};

void testSSN() {
    std::string option;
    bool exit_flag = true;
    do {
        display_option opt_1("A", "Test detection for key borad");
        printf("==================Test Toolbox======================");
        opt_1.display();
        printf("====================================================");
        scanf_s("Option(Enter ESC to exit): %c", &option);
        if (option == "A") {

        } else if (option == "ESC") {
            exit_flag = true;
        } else {
            exit_flag = false;
        }
    } while (exit_flag);

    printf("Exit!");
}