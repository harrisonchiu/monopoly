#include <player.hpp>

int Player::walk(const int steps) {
  position %= steps;
  return position;
}