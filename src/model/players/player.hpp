#ifndef PLAYER_HPP
#define PLAYER_HPP

#include "src/model/players/token.hpp"

#include "src/utils/map.hpp"

#include <fmt/color.h>
#include <fmt/format.h>

#include <array>
#include <vector>

using namespace std::literals;

class Player {
private:
  // Implicitly, @player_ids must be in range [0, 3]
  static constexpr int max_players = 4;
  static constexpr std::array<std::pair<int, Piece>, max_players> pieces{
    {
     { 1, { .character = "A", .color = Color::get("Blue") } },
     { 1, { .character = "B", .color = Color::get("Red") } },
     { 1, { .character = "C", .color = Color::get("Green") } },
     { 1, { .character = "D", .color = Color::get("Orange") } },
     }
  };
  static constexpr auto map = CompileTimeMap<int, Piece, pieces.size()>{ { pieces } };

  int id;
  std::shared_ptr<Piece> piece;

  static constexpr int starting_money = 99999;
  int money = starting_money;

  // Just because we moved the player with @Player method, doesn't mean @Board or @View
  // knows about it. @Board needs to know to move the pieces
  bool is_movement_updated = true;
  int last_pos = 0;
  int pos = 0;

public:
  // Use these methods to construct Player rather than default constructor
  explicit Player();
  explicit Player(int id);
  static auto create_multiple(int n) -> std::vector<Player>;

  static consteval auto get_max_players() -> int { return max_players; }

  constexpr auto get_id() const -> int { return id; }
  auto get_piece() const -> const Piece { return piece; }
  auto get_avatar() const -> std::string_view { return avatar; }
  auto get_character() const -> std::string_view { return piece.character; }
  auto get_color() const -> const fmt::text_style & { return piece.color; }
  auto get_money() const -> int { return money; }

  auto get_pos() const -> int { return pos; }
  auto get_last_pos() const -> int { return last_pos; }
  auto is_pos_updated() const -> bool { return is_movement_updated; }
  void set_pos_updated(bool is_updated) { is_movement_updated = is_updated; }

  void walk(int steps);
  void withdraw(int amount);
  void deposit(int amount);
};

#endif // PLAYER_HPP