#ifndef SUBSTRINGS_HPP
#define SUBSTRINGS_HPP

#include "src/view/components.hpp"

#include <array>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

// N is size of array returned or the number of substring occurrences to get. If
// N is less than or equal to the number of occurrences, the array will be the
// index of those occurrences in order. If N is greater than the number of
// occurrences, the extra elements in the array will be 0s.
template <std::size_t N>
consteval auto find_substrs(std::string_view str, std::string_view substr) {
  static_assert(N > 0, "The number of occurrences to get, N, must be positive");

  // Find all the substring's locations inside the parent string
  // Use a vector because we do not know how many occurrences there will be
  std::array<std::size_t, N> indices{};
  std::size_t pos = 0;
  int substr_find_count = 0;

  while ((pos = str.find(substr, pos)) != std::string_view::npos) {
    indices[substr_find_count] = pos;
    pos += substr.length();

    ++substr_find_count;
    if (substr_find_count >= N) {
      break;
    }
  }

  return indices;
}

// N is size of array returned or the number of substring occurrences to get. If
// N is less than or equal to the number of occurrences, the array will be the
// index of those occurrences in order. If N is greater than the number of
// occurrences, the extra elements in the array will be 0s.
// Finds (col, row) of every substring occurrence in a multi-lined string where
// lines are delimited by \n. (col, row) both start at the top left at (0, 0)
template <std::size_t N> consteval auto find_pos(std::string_view str, std::string_view substr) {
  static_assert(N > 0, "The number of occurrences to get, N, must be positive");

  std::array<Position, N> coords{};
  int last_newline = 0;
  int newline_count = 1;
  int count = 0;

  for (int i = 0; i < str.length(); ++i) {
    // Find newlines to determine the row it is on
    if (str.substr(i, 1) == "\n") {
      last_newline = i;
      ++newline_count;
    }

    // Find the substring and use its position with the previous newline to find
    // the col it starts on.
    if (str.substr(i, substr.length()) == substr) {
      coords[count] = Position{ i - last_newline, newline_count };
      ++count;
      if (count >= N) {
        break;
      }
    }
  }
  return coords;
}

// N is the number of times to repeat the string.
// Similar to std::string(int repeat_times, char character) constructor, but it
// allows for a strings as well or unicode characters
// Seems to be run during compile time only for local static vars??
// Returns array of chars, so must convert to string_view or string if needed
// NOTE: If you want a string out of this, do not use this. Prefer the standard
// string constructor `std::string(n, char)` if possible.
// If you want a string_view out of this, see:
//  constexpr std::array<char, 71*2> char_arr = repeat_str<2, 71>("12");
//  std::string_view sv(char_arr.begin(), char_arr.end());
// which uses less instructions than `std::string_view s = std::string(71, ' ')`
template <std::size_t N, std::size_t Count>
consteval auto repeat_str(std::string_view str) -> std::array<char, N * Count> {
  constexpr std::size_t out_size = N * Count;
  std::array<char, out_size> result{};

  std::array<char, N> str_chars{};
  std::copy(str.begin(), str.end(), str_chars.begin());

  auto it = result.begin();
  for (std::size_t i = 0; i < Count; ++i) {
    std::copy(str_chars.begin(), str_chars.end(), it);
    std::advance(it, N);
  }
  return result;
}

// Given a long string, split it into multiple smaller strings which are a maximum of @LineLength
// chars long and does not cut words off halfway.
// Will never be constexpr because of input and return type. Also because of our use cases.
template <std::size_t LineLength, int MaxLines>
auto split_str_into_lines(const std::string &str) -> std::vector<std::string> {
  std::vector<std::string> result;
  std::istringstream iss(str);
  std::string word;
  std::string line;

  while (iss >> word) {
    if (line.empty()) {
      line += word;
    } else if (line.length() + 1 + word.length() <= LineLength) {
      line += " " + word;
    } else {
      result.emplace_back(line);
      line = word;
    }
  }

  if (!line.empty()) {
    result.emplace_back(line);
  }

  // We assume there exists some string at N index because of how we use this to format strings.
  // It is like a default value. So create empty strings to fit size requirement if needed.
  while (result.size() < MaxLines) {
    result.emplace_back("");
  }

  return result;
}

#endif // SUBSTRINGS_HPP