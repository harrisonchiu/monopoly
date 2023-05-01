#ifndef SUBSTRINGS_HPP
#define SUBSTRINGS_HPP

#include <array>
#include <string>
#include <string_view>
#include <vector>

#include <codecvt>
#include <locale>

#include "utils/component.hpp"

// N is size of array returned. If N is less than or equal to the number of
// occurrences, the array will be the index of those occurrences in order.
// If N is greater than the number of occurrences, the extra elements in the
// array will be 0s.
template <size_t N>
constexpr auto find_substrs(std::string_view str, std::string_view substr) {
  // Find all the substring's locations inside the parent string
  // Use a vector because we do not know how many occurrences there will be
  std::vector<size_t> indices{};
  size_t position = 0;
  while ((position = str.find(substr, position)) != std::string_view::npos) {
    indices.emplace_back(position);
    position += substr.length();
  }

  // Convert the vector to an array because constexpr vectors cannot exist
  // outside of a constexpr function. A vector is not needed anyways
  std::array<size_t, N> indices_return{};
  auto iter = indices.begin();
  for (int i = 0; i < N; i++) {
    if (iter == indices.end()) {
      break;
    }
    indices_return[i] = *iter++;
  }
  return indices_return;
}

// Overloading to support other similar string types
template <size_t N>
constexpr auto find_substrs(const std::string &str, const std::string &substr) {
  return find_substrs<N>(std::string_view(str), std::string_view(substr));
}

template <size_t N>
constexpr auto find_coords(std::string_view str, std::string_view substr) {
  std::array<Position, N> coords{};
  int last_newline = 0;
  int newline_count = 1;
  int count = 0;

  for (int i = 0; i < str.size(); i++) {
    if (str.substr(i, 1).compare("\n") == 0) {
      last_newline = i;
      newline_count++;
    }

    if (str.substr(i, substr.size()).compare(substr) == 0) {
      coords[count++] = Position{i - last_newline, newline_count};
      if (count >= N) {
        break;
      }
    }
  }
  return coords;
}

#endif // SUBSTRINGS_HPP