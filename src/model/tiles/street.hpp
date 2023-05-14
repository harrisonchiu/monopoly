#ifndef STREET_HPP
#define STREET_HPP

#include "src/model/tiles/tile.hpp"

#include <nlohmann/json.hpp>

#include <string>

class Street : public Property {
  using json = nlohmann::json;

protected:
  auto get_property_status_label() const -> std::string override;

public:
  Street(const json &tile_data, int id);
  void update_detail() override;
};

#endif // STREET_HPP