#ifndef PLAYER_HPP
#define PLAYER_HPP

#include "src/model/players/token.hpp"

#include <fmt/color.h>

#include <string_view>
#include <vector>

class Player {
private:
  static constexpr int max_players = 4;

  int id; // have a copy of id in token?
  std::shared_ptr<const Token> token;

  static constexpr int starting_money = 99999;
  int money = starting_money;

  // Just because we moved the player with @Player method, doesn't mean @Board or @View
  // knows about it. @Board needs to know to move the pieces
  bool is_movement_updated = true;
  int last_pos = 0;
  int pos = 0;

public:
  explicit Player(int id);
  static auto create_multiple(int n) -> std::vector<Player>;

  static consteval auto get_max_players() -> int { return max_players; }

  auto get_id() const -> int { return id; }
  auto get_token() const -> const std::shared_ptr<const Token> & { return token; }
  auto get_color() const -> const fmt::text_style & { return token->get_color(); }
  auto get_character() const -> std::string_view { return token->get_character(); }
  auto get_avatar() const -> std::string_view { return token->get_avatar(); }

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