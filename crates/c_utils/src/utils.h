#pragma once

// copied struct from `stb_truetype.h`
struct stbtt_packedchar {
   unsigned short x0,y0,x1,y1; // coordinates of bbox in bitmap
   float xoff,yoff,xadvance;
   float xoff2,yoff2;
};

// copied struct from `stb_truetype.h`
struct stbtt_aligned_quad {
   float x0,y0,s0,t0; // top-left
   float x1,y1,s1,t1; // bottom-right
};

#define NUMBER_OF_PACKED_CHARS ('~'-' ') // ansi
/// True Type Font
struct Font {
    unsigned char *img_buffer;
    int img_width, img_height, img_channels;

    int font_size;

    struct stbtt_packedchar packed_char[NUMBER_OF_PACKED_CHARS];
};

//unsigned char * utl_image_load    (const char *filename, int *w, int *h, int *channels);
unsigned char * utl_image_load    (const unsigned char *buffer, int buffer_size, int *w, int *h, int *channels);
void            utl_image_free    (unsigned char *buffer);
int             utl_image_save_png(const char *filename, unsigned char *buffer, int w, int h, int channels);

int  utl_font_parse(unsigned char *ttf_raw, int font_size, struct Font *font);
void utl_font_free(struct Font *font);
void utl_font_get_quad(struct Font *font, char c_to_display, float *xpot, float *ypos, struct stbtt_aligned_quad *quad);

unsigned int utl_hash_one_at_time(const char *key, unsigned long len);
