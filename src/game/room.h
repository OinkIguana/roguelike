#ifndef __GAME_ROOM_H__
#define __GAME_ROOM_H__

#include <vector>
#include <memory>

namespace Game {
    class Cell;
    class Object;

    class Room {
        std::vector<std::shared_ptr<Cell>> cells;
    public:
        Room(std::vector<std::shared_ptr<Cell>>);
        std::shared_ptr<Cell> choose_cell() const;
        bool contains(std::shared_ptr<Object>) const;
        bool full() const;
        void set_visible(bool);
    };
}

#endif
