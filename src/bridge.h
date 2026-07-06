#pragma once

class QPainter;
class QPaintDevice;

struct CatppuccinColors
{
    int bg_r, bg_g, bg_b;
    int brd_r, brd_g, brd_b;
    int btn_r, btn_g, btn_b;
};

struct DecorationRect
{
    int x, y, w, h;
};

extern "C" {
typedef void (*PaintCallback)(void *painter_ptr, int width, int height);
typedef void (*MarginsCallback)(bool shadows_only, int *l, int *t, int *r, int *b);
typedef bool (*MouseCallback)(int width, double lx, double ly, int buttons);
typedef bool (*TouchCallback)(int width, double lx, double ly, bool active);
}

extern "C" {
void fill_rect(void *painter_ptr, int x, int y, int w, int h, int r, int g, int b);
}