#ifndef SUBSTRINGS_HPP
#define SUBSTRINGS_HPP

#include "utils/component.hpp"

#include <array>
#include <string>
#include <string_view>

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
  std::size_t position = 0;
  int substr_find_count = 0;

  while ((position = str.find(substr, position)) != std::string_view::npos) {
    indices[substr_find_count++] = position;
    position += substr.length();

    if (substr_find_count >= N) {
      break;
    }
  }

  return indices;
}

// Overloading to support other similar string types
template <std::size_t N>
consteval auto find_substrs(const std::string &str, const std::string &substr) {
  return find_substrs<N>(std::string_view(str), std::string_view(substr));
}

// N is size of array returned or the number of substring occurrences to get. If
// N is less than or equal to the number of occurrences, the array will be the
// index of those occurrences in order. If N is greater than the number of
// occurrences, the extra elements in the array will be 0s.
// Finds (col, row) of every substring occurrence in a multi-lined string where
// lines are delimited by \n. (col, row) both start at the top left at (0, 0)
template <std::size_t N>
consteval auto find_position(std::string_view str, std::string_view substr) {
  static_assert(N > 0, "The number of occurrences to get, N, must be positive");

  std::array<Position, N> coords{};
  int last_newline = 0;
  int newline_count = 1;
  int count = 0;

  for (int i = 0; i < str.length(); i++) {
    // Find newlines to determine the row it is on
    if (str.substr(i, 1) == "\n") {
      last_newline = i;
      newline_count++;
    }

    // Find the substring and use its position with the previous newline to find
    // the col it starts on.
    if (str.substr(i, substr.length()) == substr) {
      coords[count++] = Position{ i - last_newline, newline_count };
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

#endif // SUBSTRINGS_HPP