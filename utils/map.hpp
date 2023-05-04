#ifndef MAP_HPP
#define MAP_HPP

#include <algorithm>
#include <array>

// Map that should have compile-time constructors and getter.
// Use only for small size lookups without any inserting/deleting/modifying.
// If any of those 3 actions are used, use a normal std::(unordered_)map
template <typename Key, typename Value, std::size_t Size>
struct CompileTimeMap {
  std::array<std::pair<Key, Value>, Size> data;

  constexpr Value at(const Key &key, const Value &default_value) const {
    const auto iterator =
        std::find_if(begin(data), end(data),
                     [&key](const auto &v) { return v.first == key; });
    if (iterator != end(data)) {
      return iterator->second;
    } else {
      return default_value;
    }
  }
};

#endif // MAP_HPP