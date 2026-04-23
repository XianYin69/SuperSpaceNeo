//
// Created by EthanYan on 2026/4/9.
//

#include "testFF.h"
#include "../include/define_display_word.h"
#include "../include/display_tip.h"
#include "../testSSN.h"
#include "../../rust/cpp/ffi/rust_fileFunc/ffi.hpp"

using namespace testFF;

void testFF_Find_a_File() {
    display::define_display_word title("", "Find a File");
    display::tip tip_1("Enter a File Path (or Enter 'q' to quit):");
    std::string Path;
    do {
        title.display_title();
        tip_1.display_tip();
        std::cin >> Path;
        // use "rust/rust_fileFunc" -> get_file_info and free_file_info function

        CFileInfo* info = get_file_info(Path.c_str());
        std::cout << info -> file_name << std::endl;
        free_file_info(info);

    } while (Path.empty() || Path == "q");
}

void ::testFF::testFF() {
    display::define_display_word title("", "Test Tool for Find File");
    display::define_display_word option_1("A", "Start test Find File");
    display::define_display_word option_2("B", "Exit this Tool");
    display::tip tip_1("Enter a Option:");
    std::string option;
    do {
        title.display_title();
        option_1.display_option();
        option_2.display_option();
        std::cin >> option;
        if (option == "A") {
            testFF_Find_a_File();
        }
    } while (option != "B");
    testSSN::testSSN();
}