#ifndef TILE_HPP
#define TILE_HPP

#include "src/model/players/player.hpp"

#include "src/model/players/token.hpp"
#include "src/model/tiles/attributes.hpp"

#include <fmt/color.h>
#include <nlohmann/json.hpp>

#include <string>
#include <string_view>

enum class Action {
  Move,
  Money,
  Jail,
  Cards,
  Roll,
  Rent,
  None,
};

struct Effect {
  Action action;
  int value;
};

class Tile {
  using json = nlohmann::json;

private:
  int id;
  std::string name;
  std::string group;
  fmt::text_style color;
  std::string box;
  std::string detail;

  static constexpr int maximum_cost = 9999; // max 4 digits because of tile length
  int cost;

  bool is_ownable = false;
  OwnershipStatus ownership_status = OwnershipStatus::Unowned;
  std::shared_ptr<const Token> owner = std::make_shared<const Token>();

public:
  Tile(const json &tile_data, int id);

  auto get_id() const -> int { return id; }
  auto get_name() const -> std::string_view { return name; }
  auto get_group() const -> std::string_view { return group; }
  auto get_color() const -> const fmt::text_style & { return color; }
  auto get_box() const -> std::string_view { return box; }
  auto get_detail() const -> std::string_view { return detail; }
  void set_detail(std::string new_detail) { detail = std::move(new_detail); }
  virtual void update_detail() = 0;

  virtual auto get_card() const -> std::string_view = 0;

  static constexpr auto get_maximum_cost() -> int { return maximum_cost; }
  auto get_cost() const -> int { return cost; }

  auto get_is_ownable() const -> bool { return is_ownable; }
  auto get_ownership_status() const -> OwnershipStatus { return ownership_status; }
  void set_ownership_status(OwnershipStatus status) { ownership_status = status; }
  auto get_owner() const -> const std::shared_ptr<const Token> & { return owner; }
  auto get_owner_id() const -> int { return owner->get_id(); }
  void set_owner(const Player &player);
  auto is_owned() const -> bool;

  virtual auto get_effect() const -> const Effect & = 0;
  virtual void update_effect() = 0;
  // virtual void interact() = 0;

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

#endif // TILE_HPP