#ifndef CORNER_HPP
#define CORNER_HPP

#include "src/model/tiles/tile.hpp"

#include <nlohmann/json.hpp>

class Corner : public Event {
  using json = nlohmann::json;

private:
  // Color is the group that the tile is in. Existly solely for better appearances.
  // Card must be separated into 2 different strings: color and details section
  // They have different colors. Hard to have 1 encompassing style and some different styles in it.
  // If put as one, fmt escapes (\x1b[0m) it which cancels all colors and styles.
  static constexpr std::string_view base_color_row{ "\n{POSITION}{EMPTY_ROW_CARD_WIDTH}" };

  // Hard-coded values because its too hard to dynamically calculate width of different phrases
  // Values assume that `INDENT` is 2 spaces and card width is 33 chars wide, 18 rows
  static constexpr std::string_view base_card{ R"""(
{POSITION}{NAME:^33}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{TYPE:^33}
{POSITION}{INDENT}Tile cost: {TILE_COST: >18}{INDENT}
{POSITION}{INDENT}Is tile ownable: {IS_OWNABLE: >12}{INDENT}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{INDENT}Event type: {EVENT_TYPE: >17}{INDENT}
{POSITION}{INDENT}Event occurs for: {EVENT_OCCURENCE: >11}{INDENT}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{INDENT}{DESCRIPTION:^29}{INDENT}
{POSITION}{INDENT}{DESCRIPTION_1:^29}{INDENT}
{POSITION}{INDENT}{DESCRIPTION_2:^29}{INDENT}
{POSITION}{INDENT}{DESCRIPTION_3:^29}{INDENT}
{POSITION}{INDENT}{DESCRIPTION_4:^29}{INDENT}
{POSITION}{INDENT}{DESCRIPTION_5:^29}{INDENT}
{POSITION}{INDENT}{DESCRIPTION_6:^29}{INDENT}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
)""" };

  std::string card;

  auto create_card(const json &tile_data) -> std::string;

public:
  Corner(const json &tile_data, int id);
  void update_detail() override;

  auto get_card() const -> std::string_view override { return card; };
};

#endif // CORNER_HPP