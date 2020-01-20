#include <iostream>
#include <vector>
#include "utils.h"
#include "orbit.h"

#define Vec std::vector
#define Tuple std::tuple
#define String std::string

using std::cout;
using orbit::Orbit;

int main() {
    Vec<String> test = orbit::split_string("foo\nbar");
    return 0;
}

