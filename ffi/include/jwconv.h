#ifndef _JWCONV_H
#define _JWCONV_H

#ifdef __cplusplus
extern "C" {
#endif

char* const romaji_to_hiragana(char* const data);
char* const romaji_to_katakana(char* const data);
void string_free(char* ptr);

#ifdef __cplusplus
}
#endif

#endif
