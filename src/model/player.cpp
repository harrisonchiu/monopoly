#include "src/model/player.hpp"

#include "src/model/board.hpp"

Player::Player(int id)
    : id{ id }, piece{ pieces.at(id) },
      avatar{ fmt::format(piece.color, std::string{ piece.character }) } {}

// Does not check for players with duplicate ids. No 2 players should have the same id.
auto Player::create_single(int id) -> Player { return Player(id); }

// Creates 0 to 4 players each with unique ids.
// Ids are the order in which they are created starting from 0, ending at 3 for the last player.
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

// TODO: if steps > 40, then it does not get count the number of times it passed GO
void Player::walk(const int steps) {
  last_pos = pos;
  pos = (pos + steps) % Board::get_number_of_tiles();

  is_movement_updated = false;
}