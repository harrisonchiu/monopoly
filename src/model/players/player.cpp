#include "src/model/players/player.hpp"

#include "src/model/board.hpp"
#include "src/model/players/token.hpp"

Player::Player(const int id)
    : id{ id },
      token{ std::make_shared<const Token>(id) } {}

// Creates up to 4 players each with unique ids.
// Ids are the order in which players are created starting from 0, ending at 3.
// Ids MUST be int in range [0, 3], so we can easily identify it in std::vector<Player>
//  and its associated @Piece
auto Player::create_multiple(const int n) -> std::vector<Player> {
  std::vector<Player> players;
  players.reserve(max_players);

  // Cannot create more than the maximum number of players
  const int number_of_players = n <= max_players ? n : max_players;

  for (int id = 0; id < number_of_players; ++id) {
    Player player = Player(id);
    players.emplace_back(std::move(player));
  }

  return players;
};

// TODO: if steps > 40, then it does not get count the number of times it passed GO
void Player::walk(const int steps) {
  last_pos = pos;
  pos = (pos + steps) % Board::get_number_of_tiles();

  is_movement_updated = false;
}

void Player::withdraw(const int amount) { money -= amount; }

void Player::deposit(const int amount) { money += amount; }