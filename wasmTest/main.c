#include "canvas.h"

int main(void) {
    HTMLCanvasElement *canvas = createCanvas("myCanvas");
    canvas->setHeight(canvas, 900);
    canvas->setWidth(canvas, 1400);
    CanvasRenderingContext2D *ctx = canvas->getContext(canvas, "2d");
    ctx->strokeRect(ctx, 50, 50, 700, 300);
    freeCanvas(canvas);
    return 0;
}
