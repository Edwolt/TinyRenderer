#include "image.h"

int main(int argc, char const* argv[]) {
    Image* image = image_new(100, 100);
    Color red = {255, 0, 0};
    image_clear(image, red);
    image_save(image, "img.bmp");
    image_del(image);
    return 0;
}
