#ifndef __GAME_MAP_H__
#define __GAME_MAP_H__

#include <vector>
#include <memory>
#include <string>

namespace Game {
    class Room;
    class Cell;

    class Map {
        int w, h;
        std::vector<std::vector<std::shared_ptr<Cell>>> cells;
        std::vector<std::shared_ptr<Room>> rooms;
    public:
        Map(int);
        std::string to_string() const;
    };
}

#endif
