//
// Created by EthanYan on 2026/4/23.
//

#ifndef RUST_FILE_FUNC_HPP
#define RUST_FILE_FUNC_HPP

#include <stdint.h>
#include <stdbool.h>

#define SUCCESS 0
#define ERR_FILE_NOT_FOUND 1
#define ERR_PERMISSION_DENIED 2
#define ERR_INVALID_PATH 3
#define ERR_UNKOWN -1

#ifdef __cplusplus
extern "C" {
#endif
    typedef struct {
        const char* file_name;
        uint64_t size;
        bool is_readonly;
    } CFileInfo;

    CFileInfo* get_file_info(const char* path);

    void free_file_info(CFileInfo* ptr);

    int32_t open_a_file(const char* path);
#ifdef __cplusplus
}
#endif

#endif //RUST_FILE_FUNC_HPP