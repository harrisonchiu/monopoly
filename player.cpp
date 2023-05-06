#include "player.hpp"

auto Player::walk(const int steps) -> int {
  last_position = position;
  position %= steps;
  return position;
}