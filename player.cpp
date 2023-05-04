#include <player.hpp>

int Player::walk(const int steps) {
  last_position = position;
  position %= steps;
  return position;
}