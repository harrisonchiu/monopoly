#ifndef PLAYER_HPP
#define PLAYER_HPP

#include "src/utils/color.hpp"

#include <fmt/color.h>

#include <array>
#include <vector>

// Officially, playing pieces are called tokens
struct Token {
  // Must have default values so we can create a default Avatar
  // or also known as a no-owner Avatar
  char character{};
  fmt::text_style color = Color::none();
};

class Player {
private:
  // Implicitly, @player_ids must be in range [0, 3]
  static constexpr int max_players = 4;
  static constexpr std::array<Token, max_players> tokens{
    {{ 'A', Color::get("Blue") },
     { 'B', Color::get("Red") },
     { 'C', Color::get("Green") },
     { 'D', Color::get("Orange") }}
  };

  int id;
  Token token;

  // Ensure that piece is initialized on object creation, so this has value
  std::string avatar;

  int last_pos = 0;
  int pos = 0;

  // Just because we moved the player with @Player method, doesn't mean @Board or @View
  // knows about it. @Board needs to know to move the pieces
  bool is_movement_updated = true;

  explicit Player(int id);

public:
  static auto create_single(int id) -> Player;
  static auto create_multiple(int n) -> std::vector<Player>;

  static consteval auto get_max_players() -> int { return max_players; }

  constexpr auto get_id() const -> int { return id; }
  constexpr auto get_avatar() const -> std::string_view { return avatar; }
  constexpr auto get_character() const -> char { return token.character; }
  constexpr auto get_color() const -> const fmt::text_style & { return token.color; }

  constexpr auto get_pos() const -> int { return pos; }
  constexpr auto get_last_pos() const -> int { return last_pos; }

  constexpr auto is_pos_updated() const -> bool { return is_movement_updated; }
  constexpr void set_pos_updated(bool is_updated) { is_movement_updated = is_updated; }

  void walk(int steps);
};

#endif // PLAYER_HPP