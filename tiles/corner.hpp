#ifndef CORNER_HPP
#define CORNER_HPP

#include <string>

#include <nlohmann/json.hpp>

#include <tiles/tile.hpp>

class Corner : public Event {
  using json = nlohmann::json;

public:
  Corner(const json &tile_data, int id) : Event(tile_data, id) {}
  std::string get_detail() const override;
};

#endif // CORNER_HPP