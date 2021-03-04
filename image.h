#pragma once

#include <stdio.h>
#include <stdlib.h>
#include "defs.h"

typedef struct Color {
    u8 red;
    u8 green;
    u8 blue;
} Color;

/**
 * Manipulate a image
 */
typedef struct Image {
    i32 width;
    i32 height;
    Color* pixels;
} Image;

Image* image_new(i32 width, i32 height);
void image_del(Image* restrict image);

inline static void image_set(Image* restrict image, int x, int y, Color color) {
    image->pixels[y * image->height + x] = color;
}

void image_clear(Image* restrict image, Color color);

// Save the image as a bitmap file
bool image_save(Image* restrict image, char* path);
