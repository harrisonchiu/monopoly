#ifndef PLAYER_HPP
#define PLAYER_HPP

#include "src/utils/color.hpp"

#include <fmt/color.h>

#include <array>
#include <vector>

struct Piece {
  // Must have default values so we can create a default Avatar
  // or also known as a no-owner Avatar
  char character{};
  fmt::text_style color{ Color::none() };
};

class Player {
private:
  // Implicitly, @player_ids must be in range [0, 3]
  static constexpr int max_players = 4;
  static constexpr std::array<Piece, max_players> pieces{
    {{ 'A', Color::get("Blue") },
     { 'B', Color::get("Red") },
     { 'C', Color::get("Green") },
     { 'D', Color::get("Orange") }}
  };

  int id;
  std::shared_ptr<Piece> piece;
  std::string avatar;

  int money{};

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
  auto get_piece() const -> const std::shared_ptr<Piece> & { return piece; }
  auto get_avatar() const -> std::string_view { return avatar; }
  auto get_character() const -> char { return piece->character; }
  auto get_color() const -> const fmt::text_style & { return piece->color; }
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