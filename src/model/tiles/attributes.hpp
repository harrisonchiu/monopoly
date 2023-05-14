#ifndef ATTRIBUTE_HPP
#define ATTRIBUTE_HPP

// Could use C++20 std::concepts to determine Tile child type
// But it feels like extra overhead and unnecessary
enum class TileType { Property, Event };

enum class PropertyStatus {
  // Implicitly marks the maximum number of tiles per group
  Mortgaged,
  Unowned,
  Owned, // Basic rent | 1 owned of the group
  Tier1, // 1 house    | 2 owned of the group
  Tier2, // 2 house    | 3 owned of the group
  Tier3, // 3 house    | 4 owned of the group
  Tier4, // 4 house    | 5 owned of the group
  Tier5, // 5 house    | 6 owned of the group
};

#endif // ATTRIBUTE_HPP