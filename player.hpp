#ifndef PLAYER_HPP
#define PLAYER_HPP

#include "utils/color.hpp"

#include <fmt/color.h>

#include <array>

struct Avatar {
  // Must have default values so we can create a default Avatar
  // or also known as a no-owner Avatar
  char character{};
  fmt::text_style color = Color::none();
};

class Player {
private:
  static constexpr int max_players = 4;
  static constexpr std::array<Avatar, max_players> pieces{
    {{ 'A', Color::get("Blue") },
     { 'B', Color::get("Red") },
     { 'C', Color::get("Green") },
     { 'D', Color::get("Orange") }}
  };

  int id;
  Avatar avatar;

  // Ensure that avatar is initialized on object creation, so this has value
  std::string player = fmt::format(avatar.color, std::string{ avatar.character });

  int last_position = 0;
  int position = 0;

public:
  explicit Player(int id) : id{ id }, avatar{ pieces.at(id) } {}

  consteval auto get_id() const -> int { return id; }
  constexpr auto get_avatar() const -> Avatar { return avatar; }
  constexpr auto get_character() const -> char { return avatar.character; }
  constexpr auto get_color() const -> fmt::text_style { return avatar.color; }
  constexpr auto get_player() const -> std::string_view { return player; }

  auto walk(int steps) -> int;
};

#endif // PLAYER_HPP