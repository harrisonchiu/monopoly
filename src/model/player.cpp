#include "src/model/player.hpp"

#include "src/model/board.hpp"

Player::Player(int id)
    : id{ id }, token{ tokens.at(id) },
      avatar{ fmt::format(token.color, std::string{ token.character }) } {}

auto Player::create_single(int id) -> Player { return Player(id); }

auto Player::create_multiple(int n) -> std::vector<Player> {
  std::vector<Player> players;
  players.reserve(max_players);

  // Cannot create more than the maximum number of players
  const int number_of_players = n <= max_players ? n : max_players;

  for (int id = 0; id < number_of_players; ++id) {
    players.emplace_back(Player(id));
  }
  return players;
};

void Player::walk(const int steps) {
  last_pos = pos;
  pos = (pos + steps) % Board::get_number_of_tiles();

  is_movement_updated = false;
}