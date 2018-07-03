#include <stdio.h>

typedef struct Point {
    double x;
    double y;
} Point;

extern Point add_points(Point, Point);

int main() {
    Point first = { .x=1.2, .y=3.4 };
    Point second = { .x=-3.14, .y=42.0 };

    Point sum = add_points(first, second);

    printf("Sum is Point {x: %f, y: %f}\n", sum.x, sum.y);

    return 0;
}
