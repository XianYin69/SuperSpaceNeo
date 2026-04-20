//
// Created by EthanYan on 2026/4/16.
//

#ifndef SUPERSPACENEO_DISPLAY_TIP_H
#define SUPERSPACENEO_DISPLAY_TIP_H
#include "define_display_word.h"

#include <iostream>

namespace display {
    class tip {
    protected:
        std::string _tip;
    public:
        tip(std::string tip): _tip(tip) {};
        void display_tip() {
            std::cout << this -> _tip;
        }
        void display_tip_end() {
            std::cout << this -> _tip << std::endl;
        }
    };
}

#endif //SUPERSPACENEO_DISPLAY_TIP_H