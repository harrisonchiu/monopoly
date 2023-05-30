#include "src/model/tiles/event.hpp"

#include "src/model/board.hpp"

#include "src/utils/substrings.hpp"

#include <fmt/args.h>
#include <fmt/color.h>

#include <string>

Event::Event(const json &tile_data, const int id)
    : Tile(tile_data, id),
      card{ create_card(tile_data) },
      effect{ Effect{ actions.at(tile_data["effectType"].get<std::string>()),
                      tile_data["effectValue"].get<int>() } } {
  update_detail();
  update_effect();
}

auto Event::create_card(const json &tile_data) -> std::string {
  constexpr int card_width = 33;
  constexpr const Position &pos = Board::get_center_pos(); // cards will always be shown here
  const auto card_start = fmt::format("\x1b[{0};{1}H", pos.row, pos.col);
  const auto card_pos = fmt::arg("POSITION", fmt::format("\x1b[{}G", pos.col));
  const auto cost = tile_data.contains("cost") ? fmt::format("${}", tile_data["cost"]) : "NULL";

  const std::string color_row = fmt::format(
      get_color().has_background() ? get_color() : fmt::bg(fmt::color::white),
      base_color_row,
      card_pos,
      fmt::arg("EMPTY_ROW_CARD_WIDTH", std::string(card_width, ' '))
  );

  // 29 is the width of the description; take into account of the indents. 7 is number of lines.
  const auto description =
      split_str_into_lines<29, 7>(tile_data.value("eventDescription", "No description"));

  const std::string card_details = fmt::format(
      fmt::fg(fmt::color::black) | fmt::bg(fmt::color::white),
      base_card,
      card_pos,
      fmt::arg("INDENT", "  "),
      fmt::arg("EMPTY_ROW_CARD_WIDTH", std::string(card_width, ' ')),
      fmt::arg("NAME", tile_data["name"].get<std::string>()),
      fmt::arg("TYPE", tile_data["type"].get<std::string>()),
      fmt::arg("TILE_COST", cost),
      fmt::arg("IS_OWNABLE", get_is_ownable()),
      fmt::arg("EVENT_TYPE", tile_data["type"].get<std::string>()),
      fmt::arg("EVENT_OCCURENCE", tile_data["type"].get<std::string>()),
      fmt::arg("DESCRIPTION", "Event Description:"),
      fmt::arg("DESCRIPTION_1", description[0]),
      fmt::arg("DESCRIPTION_2", description[1]),
      fmt::arg("DESCRIPTION_3", description[2]),
      fmt::arg("DESCRIPTION_4", description[3]),
      fmt::arg("DESCRIPTION_5", description[4]),
      fmt::arg("DESCRIPTION_6", description[5]),
      fmt::arg("DESCRIPTION_7", description[6])
  );

  return fmt::format("{0}{1}{1}{1}{2}", card_start, color_row, card_details);
}

void Event::update_detail() {
  const std::string detail = fmt::format("{:^{}}", " ", Board::get_tile_length());
  set_detail(detail);
}
