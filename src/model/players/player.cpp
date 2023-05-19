#include "src/model/players/player.hpp"

#include "src/model/board.hpp"

// Creates a default non-playable player to represent no owners for tiles
Player::Player()
    : id{ -1 },
      piece{ std::make_shared<Piece>() },
      avatar{ fmt::format(piece->color, std::string{ piece->character }) } {}

Player::Player(const int id)
    : id{ id },
      piece{ std::make_shared<Piece>(pieces.at(id)) },
      avatar{ fmt::format(piece->color, std::string{ piece->character }) } {}

// Creates 0 to 4 players each with unique ids.
// Ids are the order in which they are created starting from 0, ending at 3 for the last player.
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