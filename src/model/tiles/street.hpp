#ifndef STREET_HPP
#define STREET_HPP

#include "src/model/tiles/tile.hpp"

#include "src/model/tiles/attributes.hpp"
#include "src/utils/map.hpp"

#include <nlohmann/json.hpp>

#include <array>
#include <string_view>

class Street : public Property {
  using json = nlohmann::json;
  using status_labels = std::pair<OwnershipStatus, std::string_view>;

private:
  static constexpr std::size_t status_count = static_cast<std::size_t>(OwnershipStatus::COUNT);
  static constexpr std::array<status_labels, status_count> labels = { {
      { OwnershipStatus::Mortgaged, "M" },
      { OwnershipStatus::Unowned, "_" },
      { OwnershipStatus::Owned, "X" },
      { OwnershipStatus::Tier1, "1H" },
      { OwnershipStatus::Tier2, "2H" },
      { OwnershipStatus::Tier3, "3H" },
      { OwnershipStatus::Tier4, "4H" },
      { OwnershipStatus::Tier5, "HT" },
  } };
  static constexpr auto ownership_labels =
      CompileTimeMap<OwnershipStatus, std::string_view, labels.size()>{ { labels } };

public:
  Street(const json &tile_data, int id);
  void update_detail() override;
};

#endif // STREET_HPP