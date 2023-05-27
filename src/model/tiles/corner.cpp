#include "src/model/tiles/corner.hpp"
#include "src/model/tiles/tile.hpp"

#include "src/model/board.hpp"

#include <fmt/format.h>

#include <string>

Corner::Corner(const json &tile_data, const int id)
    : Event(tile_data, id),
      card{ create_card(tile_data) } {
  update_detail();
}

void Corner::update_detail() {
  const std::string detail = fmt::format("{:^{}}", " ", Board::get_tile_length());
  set_detail(detail);
}

auto Corner::create_card([[maybe_unused]] const json &tile_data) -> std::string {
  constexpr int card_width = 33;
  const auto card_pos = fmt::arg("POSITION", "\x1b[{1}G");
  const auto cost = tile_data.contains("cost") ? fmt::format("${}", tile_data["cost"]) : "NULL";

  const std::string color_row = fmt::format(
      get_color().has_background() ? get_color() : fmt::bg(fmt::color::white),
      base_color_row,
      card_pos,
      fmt::arg("EMPTY_ROW_CARD_WIDTH", std::string(card_width, ' '))
  );

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
      fmt::arg("DESCRIPTION_1", tile_data["type"].get<std::string>()),
      fmt::arg("DESCRIPTION_2", tile_data["type"].get<std::string>()),
      fmt::arg("DESCRIPTION_3", tile_data["type"].get<std::string>()),
      fmt::arg("DESCRIPTION_4", tile_data["type"].get<std::string>()),
      fmt::arg("DESCRIPTION_5", tile_data["type"].get<std::string>()),
      fmt::arg("DESCRIPTION_6", tile_data["type"].get<std::string>())
  );

  return fmt::format("{0}{1}{1}{1}{2}", "\x1b[{0};{1}H", color_row, card_details);
}