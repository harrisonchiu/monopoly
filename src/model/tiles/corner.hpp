#ifndef CORNER_HPP
#define CORNER_HPP

#include "src/model/tiles/tile.hpp"

#include <nlohmann/json.hpp>

#include <string>

class Corner : public Event {
  using json = nlohmann::json;

public:
  Corner(const json &tile_data, int id);
  void update_detail() override;
};

#endif // CORNER_HPP