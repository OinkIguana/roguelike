#include "room.h"

namespace Game {
    Room::Room(std::vector<std::shared_ptr<Cell>> cells) : cells{ cells } {}
}
