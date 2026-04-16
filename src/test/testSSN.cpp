//
// Created by EthanYan on 2026/4/9.
//
#include <stdio.h>
#include <iostream>
#include "testSSN.h"
#include "./testFF/testFF.h"
#include "./include/define_display_word.h"
#include "./include/display_tip.h"
//It is a menu to run testTool

namespace testSSN {
    void testSSN() {
        std::string option;
        bool exit_flag = true;
        do {
            display::define_display_word title("", "Test SSN");
            display::tip tip_1("Enter a option (ESC to exit):");
            display::define_display_word opt_1("A", "Test detection for key borad");
            title.display_title();
            opt_1.display_option();
            tip_1.display_tip();
            std::cin >> option;
            if (option == "A") {
                testFF::testFF();
            } else if (option == "ESC") {
                exit_flag = true;
            } else {
                exit_flag = false;
            }
        } while (exit_flag);

        printf("Exit!");
    }
}