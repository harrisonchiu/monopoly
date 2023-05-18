#ifndef TILE_HPP
#define TILE_HPP

#include "src/model/player.hpp"
#include "src/model/tiles/attributes.hpp"

#include <nlohmann/json.hpp>

#include <string>

class Tile {
  using json = nlohmann::json;

private:
  int id;
  std::string name;
  std::string group;
  std::string detail;

  int cost = 0;
  int owner_id = -1;
  std::shared_ptr<Piece> owner_marker = std::make_shared<Piece>();
  bool is_ownable = false;
  OwnershipStatus ownership_status = OwnershipStatus::Unowned;

public:
  Tile(const json &tile_data, int id);

  constexpr auto get_id() const -> int { return id; }
  constexpr auto get_name() const -> std::string_view { return name; }
  constexpr auto get_group() const -> std::string_view { return group; }
  constexpr auto get_detail() const -> std::string_view { return detail; }
  constexpr void set_detail(std::string new_detail) { detail = std::move(new_detail); }
  virtual void update_detail() = 0;

  auto get_cost() const -> int { return cost; }
  void set_owner(Player &player);
  auto get_owner_id() const -> int { return owner_id; }
  auto get_owner_marker() const -> const std::shared_ptr<Piece> & { return owner_marker; }
  auto get_is_ownable() const -> bool { return is_ownable; }
  auto get_ownership_status() const -> OwnershipStatus { return ownership_status; }

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