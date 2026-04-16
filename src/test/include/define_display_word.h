//
// Created by EthanYan on 2026/4/16.
//

#ifndef SUPERSPACENEO_DEFINE_DISPLAY_WORD_H
#define SUPERSPACENEO_DEFINE_DISPLAY_WORD_H

#include <iostream>

namespace display {
    class define_display_word {
    protected:
        std::string option;
        std::string title;
    public:
        define_display_word(std::string option, std::string title):
        option(option), title(title) {};
        void display_option() {
            std::cout << this -> option << ".\t" << this -> title << std::endl;
        }
        void display_title() {
            std::cout << "\n\t=========" << this -> title << "\t=========" << std::endl;
        }
    };
}

#endif //SUPERSPACENEO_DEFINE_DISPLAY_WORD_H