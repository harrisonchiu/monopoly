#ifndef MAP_HPP
#define MAP_HPP

#include <algorithm>
#include <array>
#include <stdexcept>

// Map that should have compile-time constructors and getter.
// Use only for small size lookups without any inserting/deleting/modifying.
// If any of those 3 actions are used, use a normal std::(unordered_)map
template <typename Key, typename Value, std::size_t Size> struct CompileTimeMap {
  // Disable linting misc-non-private-member-variables-in-classes because I dont care
  std::array<std::pair<Key, Value>, Size> data; // NOLINT

  // General value getter; use if unsure or want a default value. Prefer this over @at()
  // Returns default value if @key is not found in the map.
  constexpr auto get(const Key &key, const Value &default_value) const -> Value {
    const auto iterator =
        std::find_if(begin(data), end(data), [&key](const auto &v) { return v.first == key; });
    if (iterator != end(data)) {
      return iterator->second;
    }
    return default_value;
  }

  // Use if you know the key exist.
  // Exactly like @get() but without default value and will throw a range error
  constexpr auto at(const Key &key) const -> Value {
    const auto iterator =
        std::find_if(begin(data), end(data), [&key](const auto &v) { return v.first == key; });
    if (iterator != end(data)) {
      return iterator->second;
    }
    throw std::range_error("Key does not exist in CompileTimeMap");
  }
};

#endif // MAP_HPP