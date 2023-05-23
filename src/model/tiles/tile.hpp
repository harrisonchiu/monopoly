#ifndef TILE_HPP
#define TILE_HPP

#include "src/model/players/player.hpp"

#include "src/model/players/token.hpp"
#include "src/model/tiles/attributes.hpp"

#include <nlohmann/json.hpp>

#include <string>

class Tile {
  using json = nlohmann::json;

private:
  int id;
  std::string name;
  std::string group;
  std::string color;
  std::string detail;

  // Cannot be more than 4 digits because of tile length
  static constexpr int maximum_cost = 9999;
  int cost;

  bool is_ownable;
  OwnershipStatus ownership_status = OwnershipStatus::Unowned;
  std::shared_ptr<const Token> owner = std::make_shared<const Token>();

public:
  Tile(const json &tile_data, int id);

  auto get_id() const -> int { return id; }
  auto get_name() const -> std::string_view { return name; }
  auto get_group() const -> std::string_view { return group; }
  auto get_color() const -> std::string_view { return color; }
  auto get_detail() const -> std::string_view { return detail; }
  void set_detail(std::string new_detail) { detail = std::move(new_detail); }
  virtual void update_detail() = 0;

  auto get_cost() const -> int { return cost; }
  auto get_is_ownable() const -> bool { return is_ownable; }
  auto get_ownership_status() const -> OwnershipStatus { return ownership_status; }

  auto get_owner() const -> const std::shared_ptr<const Token> & { return owner; }
  auto get_owner_id() const -> int { return owner->get_id(); }
  void set_owner(const Player &player);

  // Special member functions defined for Rule of Five to get rid of warnings
  Tile(const Tile &) = delete; // Copy
  auto operator=(const Tile &) -> Tile & = delete;
  Tile(const Tile &&) = delete; // Move
  auto operator=(const Tile &&) -> Tile & = delete;
  virtual ~Tile() = default; // Destructor
};

// Property abstract class
class Property : public Tile {
  using json = nlohmann::json;

public:
  Property(const json &tile_data, int id);
};

// Event abstract class
class Event : public Tile {
  using json = nlohmann::json;

public:
  Event(const json &tile_data, int id);
};

#endif // TILE_HPP