#ifndef MAP_HPP
#define MAP_HPP

#include <algorithm>
#include <array>

// Map that should have compile-time constructors and getter.
// Use only for small size lookups without any inserting/deleting/modifying.
// If any of those 3 actions are used, use a normal std::(unordered_)map
template <typename Key, typename Value, std::size_t Size> struct CompileTimeMap {
  // Disable linting misc-non-private-member-variables-in-classes because I dont care
  std::array<std::pair<Key, Value>, Size> data; // NOLINT

  constexpr auto at(const Key &key, const Value &default_value) const -> Value {
    const auto iterator =
        std::find_if(begin(data), end(data), [&key](const auto &v) { return v.first == key; });
    if (iterator != end(data)) {
      return iterator->second;
    }
    return default_value;
  }
};

#endif // MAP_HPP