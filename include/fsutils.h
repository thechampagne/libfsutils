#ifndef __FS_UTILS_H__
#define __FS_UTILS_H__

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef union {
  char* buffer;
  char* error;
} fs_utils_t;

/*
* Checks if the given folder is empty.
* 
* Example:
* * *
* int main()
* {
*   int is_empty;
*   if (fs_utils_is_folder_empty(".", &is_empty) != 0)
*   {
*     printf("Something went wrong\n");
*     return -1;
*   }
*   printf("Is folder empty: %d\n", is_empty);
*   return 0;
* }
* * *
* 
* @param path
* @return 0 on success and non zero value on failure
*/
extern int fs_utils_is_folder_empty(const char* path, int* is_empty);

/*
* Copies the contents of the source directory to the given destination directory.
* In destination_dir, a new subdirectory with the basename of the source_dir will be created.
* It will not perform the copy operation if the effective destination directory does already exist.
* 
* Example:
* * *
* int main()
* {
*   fs_utils_t fs;
*   int res;
*   if ((res = fs_utils_copy_directory(&fs,"src", "dest")) != 0)
*   {
*     if (res == 1)
*     {
*       printf("Something went wrong: %s", fs.error);
*       return -1;
*     }
*     else
*     {
*       printf("Something went wrong");
*       return -1;
*     }
*   }
*   
*   printf("Path: %s\n", fs.buffer);
*   fs_utils_clean(&fs);
*   return 0;
* }
* * *
* 
* @param fs_utils pointer to fs_utils_t
* @param source_dir
* @param destination_dir
* @return 0 on success and non zero value on failure
*/
extern int fs_utils_copy_directory(fs_utils_t* fs_utils, const char* source_dir, const char* destination_dir);

/*
* Example:
* * *
* int main()
* {
*   char* res = fs_utils_destination_directory("src", "dest");
*   if (res == NULL)
*   {
*       printf("Something went wrong");
*       return -1;
*   }
*   
*   printf("Destination directory: %s\n", res);
*   fs_utils_free(res);
*   return 0;
* }
* * *
* 
* @param source_dir
* @param destination_dir
* @return the computed destination directory, given a source directory.
*/
extern char* fs_utils_destination_directory(const char* source_dir, const char* destination_dir);

/*
* Reads the first N bytes from a file.
* It is equivalent to head -c limit *nix utility.
*
* Example:
* * *
* int main()
* {
*   size_t length;
*   uint8_t* res = fs_utils_head("path", 10, &length);
*   if (res == NULL)
*   {
*       printf("Something went wrong");
*       return -1;
*   }
*   for (size_t i = 0; i < length; i++)
*   {
*      printf("%c", res[i]);
*   }
*   fs_utils_free_array(res, length);
*   return 0;
* }
* * *
* 
* @param path
* @param limit
* @param length
* @return array
*/
extern uint8_t* fs_utils_head(const char* path, size_t limit, size_t* length);

/*
* Reads the first N bytes from a file and return them as a string.
* It assumes that the file is encoded with UTF-8, so any invalid UTF-8
* sequences will be replaced with U+FFFD REPLACEMENT CHARACTER, which looks like this: �.
*
* Example:
* * *
* int main()
* {
*   char* res = fs_utils_head_to_string("path", 10);
*   if (res == NULL)
*   {
*       printf("Something went wrong");
*       return -1;
*   }
*   printf("%s", res);
*   fs_utils_free(res);
*   return 0;
* }
* * *
* 
* @param path
* @param limit
* @return dynamic string
*/
extern char* fs_utils_head_to_string(const char* path, size_t limit);

/*
* Reads the first N bytes from a file and return them as a string.
* If the file size is greater than N bytes, the truncation message will be put at the end of the String.
* It assumes that the file is encoded with UTF-8, so any invalid UTF-8
* sequences will be replaced with U+FFFD REPLACEMENT CHARACTER, which looks like this: �.
*
* Example:
* * *
* int main()
* {
*   char* res = fs_utils_head_to_string_with_message("path", 10, "Error");
*   if (res == NULL)
*   {
*       printf("Something went wrong");
*       return -1;
*   }
*   printf("%s", res);
*   fs_utils_free(res);
*   return 0;
* }
* * *
* 
* @param path
* @param limit
* @param truncation_message
* @return dynamic string
*/
extern char* fs_utils_head_to_string_with_message(const char* path, size_t limit, const char* truncation_message);

/*
* Cleans up the contents (files and folders) of the given folder while keeping the folder itself.
* It is useful if you don't want to loose the permissions set on the folder
* or if you only have enough permissions to manipulate with the contents of the given folder
* but not the folder itself.
*
* Example:
* * *
* int main()
* {
*   if (fs_utils_cleanup_folder("folder_path") != 0)
*   {
*       printf("Something went wrong");
*       return -1;
*   }
*   return 0;
* }
* * *
* 
* @param folder_path
* @return 0 on success and non zero value on failure
*/
extern int fs_utils_cleanup_folder(const char* folder_path);

/*
* function to free the memory after using fs_utils functions
*
* @param ptr string returned from fs_utils functions
*/
extern void fs_utils_free(char* ptr);

/*
* function to free the memory after using fs_utils functions
*
* @param ptr array returned from fs_utils functions
* @param length array length
*/
extern void fs_utils_free_array(uint8_t* ptr, size_t length);

/*
* function to free the memory after using fs_utils functions
*
* @param ptr pointer to fs_utils_t
*/
extern void fs_utils_clean(fs_utils_t* ptr);


#ifdef __cplusplus
}
#endif

#endif // __FS_UTILS_H__