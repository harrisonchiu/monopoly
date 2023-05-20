#include "src/model/tiles/street.hpp"
#include "src/model/tiles/tile.hpp"

#include "src/model/board.hpp"

#include <fmt/core.h>
#include <nlohmann/json.hpp>

#include <string>
#include <string_view>

Street::Street(const json &tile_data, const int id)
    : Property(tile_data, id) {
  update_detail();
}

void Street::update_detail() {
  const std::string_view label = ownership_labels.at(get_ownership_status());
  const std::string detail = fmt::format(get_owner_marker().color, label);
  const std::string cost = fmt::format("${}", get_cost());

  const int max_cost_length = Board::get_tile_length() - static_cast<int>(label.length());

  const std::string new_detail =
      fmt::format("{}{:>{LENGTH}}", detail, cost, fmt::arg("LENGTH", max_cost_length));

  set_detail(new_detail);
}