#ifndef STREET_HPP
#define STREET_HPP

#include "src/model/tiles/tile.hpp"

#include <nlohmann/json.hpp>

#include <array>
#include <string>
#include <string_view>

class Street : public Property {
  using json = nlohmann::json;

private:
  // Color is the group that the tile is in. Existly solely for better appearances.
  // Card must be separated into 2 different strings: color and details section
  // They have different colors. Hard to have 1 encompassing style and some different styles in it.
  // If put as one, fmt escapes (\x1b[0m) it which cancels all colors and styles.
  static constexpr std::string_view base_color_row{ "\n{POSITION}{EMPTY_ROW_CARD_WIDTH}" };

  // Hard-coded values because its too hard to dynamically calculate width of different phrases
  // Values assume that `INDENT` is 2 spaces and card width is 33 chars wide, 20 rows
  static constexpr std::string_view base_card{ R"""(
{POSITION}{NAME:^33}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{TYPE:^33}
{POSITION}{INDENT}Tile cost: {TILE_COST: >18}{INDENT}
{POSITION}{INDENT}Mortgage value: {MORTGAGE_VALUE: >13}{INDENT}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{INDENT}Basic rent: {RENT_BASIC: >17}{INDENT}
{POSITION}{INDENT}Rent with full set: {RENT_FULLSET: >9}{INDENT}
{POSITION}{INDENT}Rent with 1 house: {RENT_1HOUSE: >10}{INDENT}
{POSITION}{INDENT}Rent with 2 houses: {RENT_2HOUSE: >9}{INDENT}
{POSITION}{INDENT}Rent with 3 houses: {RENT_3HOUSE: >9}{INDENT}
{POSITION}{INDENT}Rent with 4 houses: {RENT_4HOUSE: >9}{INDENT}
{POSITION}{INDENT}Rent with hotel: {RENT_HOTEL: >12}{INDENT}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{INDENT}House cost: {HOUSE_COST: >17}{INDENT}
{POSITION}{INDENT}Hotel cost: {HOTEL_COST: >17}{INDENT}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
)""" };

  std::string card;

  std::vector<int> rent;
  int current_rent = 0;

  Effect effect{};

  auto create_card(const json &tile_data) -> std::string;

public:
  Street(const json &tile_data, int id);
  void update_detail() override;

  auto get_card() const -> std::string_view override { return card; };

  auto get_effect() const -> const Effect & override { return effect; };
  void update_effect() override;
};

#endif // STREET_HPP