#ifndef TILE_HPP
#define TILE_HPP

#include <string>

#include <nlohmann/json.hpp>

#include <player.hpp>

typedef nlohmann::json json;

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

class Tile {
private:
  const int id;
  std::string name;
  std::string group;

public:
  Tile(const json &tile_data, int id) : id{id} {
    name = tile_data["name"];
    group = tile_data["group"];
  }

  virtual constexpr int get_id() const { return id; }
  virtual constexpr std::string get_name() const { return name; }
  virtual constexpr std::string get_group() const { return group; }
  virtual consteval TileType get_type() const = 0;
  virtual std::string get_detail() const = 0;

  //   virtual void visited_by() = 0;
  //   virtual void view() = 0;
  //   virtual void interact() = 0;
  //   virtual void monopolize() = 0;
  //   virtual void change_owner() = 0;
  virtual ~Tile() {}
};

class Property : public Tile {
private:
  static constexpr Avatar no_owner{};

protected:
  int property_cost;
  PropertyStatus property_status = PropertyStatus::Unowned;
  std::unique_ptr<Avatar> owner = std::make_unique<Avatar>(no_owner);

  virtual std::string get_property_status_label() const = 0;

public:
  Property(const json &tile_data, int id) : Tile(tile_data, id) {
    property_cost = tile_data["property_cost"];
  }
  consteval TileType get_type() const override { return TileType::Property; }
  virtual ~Property() {}
};

class Event : public Tile {
public:
  Event(const json &tile_data, int id) : Tile(tile_data, id) {}
  consteval TileType get_type() const override { return TileType::Event; }
  virtual ~Event() {}
};

#endif // TILE_HPP