#include <assert.h>
#include <malloc.h>
#include <memory.h>

//#define STB_IMAGE_STATIC
#define STB_IMAGE_IMPLEMENTATION
#include "stb/stb_image.h"

//#define STB_IMAGE_WRITE_STATIC
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb/stb_image_write.h"

//#define STBRP_STATIC
#define STB_RECT_PACK_IMPLEMENTATION
#include "stb/stb_rect_pack.h"

//#define STBTT_STATIC
#define STB_TRUETYPE_IMPLEMENTATION
#include "stb/stb_truetype.h"

#include "stb/stb_vorbis.c"

#include "utils.h"

unsigned char *
utl_image_load(const unsigned char *buffer, int buffer_size, int *w, int *h, int *channels)
{
    return stbi_load_from_memory(buffer, buffer_size, w, h, channels, 0);
}

int
utl_image_save_png(const char *filename, unsigned char *buffer, int w, int h, int channels)
{
     return stbi_write_png(filename, w, h, channels, buffer, w);
}

void
utl_image_free(unsigned char *buffer)
{
    stbi_image_free(buffer);
}

int
utl_font_parse(unsigned char *ttf_raw, int font_size, struct Font *font)
{
    assert(ttf_raw && "passing null as raw");
    assert(font && "pleas pass allocated font before calling util_font_pars(..)!");
    font->img_width    = 1024;
    font->img_height   = 1024;
    font->img_channels = 1;
    font->font_size  = font_size;
    font->img_buffer = malloc(font->img_width * font->img_height * font->img_channels);
    assert(font->img_buffer && "Failed to allocate mem for pixels.");

    int success = 0;
    stbtt_pack_context spc;
    success = stbtt_PackBegin(&spc, font->img_buffer, font->img_width, font->img_height, font->img_width, 1, NULL);
    assert(success);

    stbtt_PackSetOversampling(&spc, 8, 8);

    success = stbtt_PackFontRange(&spc, ttf_raw, 0, STBTT_POINT_SIZE(font->font_size),
            ' ', NUMBER_OF_PACKED_CHARS, (stbtt_packedchar *)&font->packed_char[0]);
    assert(success);

    stbtt_PackEnd(&spc);

    return 1;
}

void
utl_font_free(struct Font *font)
{
    assert(font && "passing invalid font to free");
    if (font->img_buffer != NULL)
        free(font->img_buffer);
}

void
utl_font_get_quad(struct Font *font, char c_to_display, float *xpos, float *ypos, struct stbtt_aligned_quad *quad)
{
    assert(font);
    stbtt_GetPackedQuad((stbtt_packedchar*)font->packed_char, font->img_width, font->img_height,
                        (c_to_display - ' '),     // character to display
                        xpos, ypos,               // pointers to current position in screen pixel space
                        (stbtt_aligned_quad*)quad,// output: quad to draw
                        1);
}

unsigned int
utl_hash_one_at_time(const char *key, unsigned long len)
{
    unsigned int hash = 0;
    for ( unsigned long i = 0
        ; i < len
        ; i ++ )
    {
        hash += key[i];
        hash += hash << 10;
        hash ^= hash >> 6;
    }

    hash += hash << 3;
    hash ^= hash >> 11;
    hash += hash << 15;

    return hash;
}
