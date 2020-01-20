#include <tuple>
#include <vector>

#define Vec std::vector
#define Tuple std::tuple
#define String std::string

namespace orbit {
    typedef Tuple<String, String> Orbit;

    Vec<String> split_string(String str);
    Orbit parse_sentence();
}

/// Parses a string and returns a vector with line-separated strings.
Vec<String> split_string(String str) {
    Vec<String> tokens;
    Vec<char> chars(str.begin(), str.end());
    String queue;

    for (int i = 0; ; i++) {
	if (i != chars.size()) {
	    queue.push_back(chars[i]);
	}

	if (i == chars.size() || chars[i] == '\n') {
	    tokens.push_back(queue);
	    queue = String("");
	}
    }

    return tokens;
}
