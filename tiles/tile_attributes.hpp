#ifndef TILE_ATTRIBUTES_HPP
#define TILE_ATTRIBUTES_HPP

enum class TileType { Property, Event };

enum class PropertyStatus {
  Mortgaged,
  Unowned,
  Owned, // Basic rent | 1 owned of the set
  Tier1, // 1 house    | 2 owned of the set
  Tier2, // 2 house    | 3 owned of the set
  Tier3, // 3 house    | 4 owned of the set
  Tier4, // 4 house    | 5 owned of the set
  Tier5, // 5 house    | 6 owned of the set
};

#endif // TILE_ATTRIBUTES_HPP