#ifndef SORTING_HPP
#define SORTING_HPP

#include <array>

template <typename T, std::size_t Size>
constexpr auto sort_by_order(std::array<T, Size> data, std::array<int, Size> order) {
  // O(N) sorting algorithm based on the order of a given array
  // Copies all the values and creates a new array.
  // @data: array to be sorted based on @order
  // @order: array values determine the order of @data's elements. Its domain
  // must be continuous [0, @indices.len), strictly increasing by 1.
  // i.e. NO gaps Ex: fn([a, b, c, d], [0, 3, 2, 1]) -> [a, d, c, b]

  std::array<T, Size> sorted{};
  for (std::size_t i = 0; i < Size; ++i) {
    sorted[order[i]] = data[i];
  }

  return sorted;
}

#endif // SORTING_HPP