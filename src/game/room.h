#ifndef __GAME_ROOM_H__
#define __GAME_ROOM_H__

#include <vector>
#include <memory>

namespace Game {
    class Cell;

    class Room {
        std::vector<std::shared_ptr<Cell>> cells;
    public:
        Room(std::vector<std::shared_ptr<Cell>>);
    };
}

#endif
