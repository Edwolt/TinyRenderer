#include "image.h"

Image* image_new(i32 width, i32 height) {
    Image* restrict image = malloc(sizeof(Image));
    if (!image) goto error;

    image->pixels = NULL;

    image->width = width;
    image->height = height;
    image->pixels = malloc(width * height * sizeof(Color));
    if (!image->pixels) goto error;

    return image;

error:
    image_del(image);
    return NULL;
}

void image_del(Image* restrict image) {
    if (!image) return;
    free(image->pixels);
    free(image);
}

void image_clear(Image* restrict image, Color color) {
    for (int i = 0; i < image->width * image->height; i++) {
        image->pixels[i] = color;
    }
}

bool image_save(Image* restrict image, char* path) {
    FILE* restrict file = NULL;

    file = fopen(path, "wb");
    if (!file) goto error;

    const u32 header_size = 14;
    const u32 dib_size = 40;
    const u32 image_size = 3 * image->width * image->height;

    // * Header
    // Indentify the file
    const char* restrict type = "BM";
    fwrite(type, 2 * sizeof(char), 1, file);
    if (ferror(file)) goto error;

    // File size
    const u32 size = header_size + dib_size + image_size;
    fwrite(&size, sizeof(u32), 1, file);
    if (ferror(file)) goto error;

    // Unused two fields of 2 bytes
    u16 reserved = 0;  // Must be 0
    fwrite(&reserved, sizeof(u16), 1, file);
    if (ferror(file)) goto error;
    fwrite(&reserved, sizeof(u16), 1, file);
    if (ferror(file)) goto error;

    // Offset where the image can be found
    const u32 offset = header_size + dib_size;
    fwrite(&offset, sizeof(u32), 1, file);
    if (ferror(file)) goto error;

    // * DIB Header (Image Header)

    // DIB file Size
    fwrite(&dib_size, sizeof(u32), 1, file);
    if (ferror(file)) goto error;

    // width and height
    fwrite(&image->width, sizeof(i32), 1, file);
    if (ferror(file)) goto error;
    fwrite(&image->height, sizeof(i32), 1, file);
    if (ferror(file)) goto error;

    // Number of color planes
    const u16 color_planes = 1;  // Must be 1
    fwrite(&color_planes, sizeof(u16), 1, file);
    if (ferror(file)) goto error;

    // Color depth
    const u16 color_depth = 24;
    fwrite(&color_depth, sizeof(u16), 1, file);
    if (ferror(file)) goto error;

    // Compression method
    const u32 compression = 0;  // None
    fwrite(&compression, sizeof(u32), 1, file);
    if (ferror(file)) goto error;

    // Image size (None compression can use a dummy 0)
    fwrite(&image_size, sizeof(u32), 1, file);
    if (ferror(file)) goto error;

    // horizontal and vertical pixel per meter
    const i16 pixel_meter = 0;                   // No preference
    fwrite(&pixel_meter, sizeof(i16), 1, file);  // horizontal
    if (ferror(file)) goto error;
    fwrite(&pixel_meter, sizeof(i16), 1, file);  // vertical
    if (ferror(file)) goto error;

    // Number of color used
    const u32 palette = 0;  // It isn't using a palette
    fwrite(&palette, sizeof(u32), 1, file);
    if (ferror(file)) goto error;

    // Number of important colors of the palette
    const u32 important_color = 0;  // No palette
    fwrite(&important_color, sizeof(u32), 1, file);
    if (ferror(file)) goto error;

    // * Image

    for (int i = 0; i < image->width * image->height; i++) {
        fwrite(&image->pixels[i].red, sizeof(u8), 1, file);
        if (ferror(file)) goto error;
        fwrite(&image->pixels[i].green, sizeof(u8), 1, file);
        if (ferror(file)) goto error;
        fwrite(&image->pixels[i].blue, sizeof(u8), 1, file);
        if (ferror(file)) goto error;
    }

    fclose(file);
    return true;
error:
    fclose(file);
    return false;
}
