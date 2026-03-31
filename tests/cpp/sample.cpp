#include <cmath>
#include <string>

class Point {
public:
    double x;
    double y;

    Point(double x, double y) : x(x), y(y) {}

    double distance(const Point& other) const {
        return std::sqrt(std::pow(x - other.x, 2) + std::pow(y - other.y, 2));
    }

    static Point of(double x, double y) {
        return Point(x, y);
    }
};

class Circle {
public:
    Point center;
    double radius;

    Circle(Point center, double radius) : center(center), radius(radius) {}

    double area() const {
        return 3.14159265 * radius * radius;
    }

    static Circle of(Point center, double radius) {
        return Circle(center, radius);
    }
};
