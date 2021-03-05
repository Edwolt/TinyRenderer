#pragma once

#include <stdio.h>
#include <stdlib.h>
#include "defs.h"

/**
 * Store a point
 */
typedef struct Point {
    i32 x;
    i32 y;
} Point;

/**
 * Stores a RGB color 
 */
typedef struct Color {
    u8 red;
    u8 green;
    u8 blue;
} Color;

/**
 * Store a image
 * and allow to manipulate it
 */
typedef struct Image {
    i32 width;
    i32 height;
    Color* pixels;
} Image;

/**
 * Allocate image struct
 */
Image* image_new(i32 width, i32 height);

/**
 * Delete image from memory and
 * free the memory
 */
void image_del(const Image* restrict image);

/**
 * Set the value of pixel at (x, y) to color
 */
inline static void image_set(Image* restrict image, i32 x, i32 y, Color* const color) {
    image->pixels[y * image->height + x] = *color;
}

void image_line(Image* restrict image, Point* const p0, Point* const p2);

/**
 * Set all pixels of image to color
 */
void image_clear(Image* restrict image, const Color* restrict color);

/**
 * Save the image as a bitmap file
 */
bool image_save(Image* restrict image, char* path);
