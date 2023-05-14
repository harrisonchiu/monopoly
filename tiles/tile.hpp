#ifndef TILE_HPP
#define TILE_HPP

#include "player.hpp"
#include "tiles/tile_attributes.hpp"

#include <nlohmann/json.hpp>

#include <string>

class Tile {
  using json = nlohmann::json;

private:
  int id;
  std::string name;
  std::string group;
  std::string detail;

public:
  Tile(const json &tile_data, int id)
      : id{ id }, name{ tile_data["name"] }, group{ tile_data["group"] } {}

  constexpr auto get_id() const -> int { return id; }
  constexpr auto get_name() const -> std::string_view { return name; }
  constexpr auto get_group() const -> std::string_view { return group; }
  constexpr auto get_detail() const -> std::string_view { return detail; }
  constexpr auto set_detail(std::string new_detail) -> void { detail = std::move(new_detail); }
  virtual void update_detail() = 0;
  virtual constexpr auto get_type() const -> TileType = 0;

  //   virtual void visited_by() = 0;
  //   virtual void view() = 0;
  //   virtual void interact() = 0;
  //   virtual void monopolize() = 0;
  //   virtual void change_owner() = 0;

  // Special member functions defined for Rule of Five to get rid of warnings
  Tile(const Tile &) = delete; // Copy
  auto operator=(const Tile &) -> Tile & = delete;
  Tile(const Tile &&) = delete; // Move
  auto operator=(const Tile &&) -> Tile & = delete;
  virtual ~Tile() = default; // Destructor
};

class Property : public Tile {
  using json = nlohmann::json;

private:
  static constexpr Token no_owner{};
  int property_cost;
  PropertyStatus property_status = PropertyStatus::Unowned;
  std::shared_ptr<Token> owner = std::make_shared<Token>(no_owner);

protected:
  auto get_property_cost() const -> int { return property_cost; }
  auto get_property_status() const -> PropertyStatus { return property_status; }
  virtual auto get_property_status_label() const -> std::string = 0;
  auto get_owner() const -> std::shared_ptr<Token> { return owner; }

public:
  Property(const json &tile_data, int id)
      : Tile(tile_data, id), property_cost{ tile_data["property_cost"] } {}
  constexpr auto get_type() const -> TileType override { return TileType::Property; }

  // Special member functions defined for Rule of Five to get rid of warnings
  Property(const Property &) = delete; // Copy
  auto operator=(const Property &) -> Property & = delete;
  Property(const Property &&) = delete; // Move
  auto operator=(const Property &&) -> Property & = delete;
  ~Property() override = default; // Destructor
};

class Event : public Tile {
  using json = nlohmann::json;

public:
  Event(const json &tile_data, int id) : Tile(tile_data, id) {}
  constexpr auto get_type() const -> TileType override { return TileType::Event; }

  // // Special member functions defined for Rule of Five to get rid of warnings
  Event(const Event &) = delete; // Copy
  auto operator=(const Event &) -> Event & = delete;
  Event(const Event &&) = delete; // Move
  auto operator=(const Event &&) -> Event & = delete;
  ~Event() override = default; // Destructor
};

#endif // TILE_HPP