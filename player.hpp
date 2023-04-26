#ifndef PLAYER_HPP
#define PLAYER_HPP

#include <array>

#include <fmt/color.h>
#include <fmt/core.h>

#include <utils/color.hpp>

struct Avatar {
  // Must have default values so we can create a default Avatar
  // or also known as a no-owner Avatar
  const char character{};
  const fmt::text_style color = Color::none();
};

class Player {
private:
  static constexpr int max_players = 4;
  static constexpr std::array<Avatar, max_players> pieces = {
      {{'A', Color::get("Blue")},
       {'B', Color::get("Red")},
       {'C', Color::get("Green")},
       {'D', Color::get("Orange")}}};

  const int id;
  const Avatar avatar;

  int position = 0;

public:
  Player(int id) : id{id}, avatar{pieces[id]} {}

  consteval int get_id() { return id; }
  constexpr Avatar get_avatar() { return avatar; }
  constexpr char get_character() { return avatar.character; }
  constexpr fmt::text_style get_color() { return avatar.color; }
  constexpr std::string get_player() {
    return fmt::format(avatar.color, std::string{avatar.character});
  }

  int walk(int steps);
};

#endif // PLAYER_HPP