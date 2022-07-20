# libfsutils

[![](https://img.shields.io/github/v/tag/thechampagne/libfsutils?label=version)](https://github.com/thechampagne/libfsutils/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libfsutils)](https://github.com/thechampagne/libfsutils/blob/main/LICENSE)

Utilities to help working with the filesytem for **C**.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libfsutils.git
```
#### 2. Navigate to the root
```
cd libfsutils
```
#### 3. Build the project
```
cargo build
```

### Available functions

```c
int fs_utils_is_folder_empty(const char* path, int* is_empty);

int fs_utils_copy_directory(fs_utils_t* fs_utils, const char* source_dir, const char* destination_dir);

char* fs_utils_destination_directory(const char* source_dir, const char* destination_dir);

uint8_t* fs_utils_head(const char* path, size_t limit, size_t* length);

char* fs_utils_head_to_string(const char* path, size_t limit);

char* fs_utils_head_to_string_with_message(const char* path, size_t limit, const char* truncation_message);

int fs_utils_cleanup_folder(const char* folder_path);

void fs_utils_free(char* ptr);

void fs_utils_free_array(uint8_t* ptr, size_t length);

void fs_utils_clean(fs_utils_t* ptr);
```

### References
 - [fs-utils](https://github.com/Byron/fs-utils-rs)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libfsutils/blob/main/LICENSE).
