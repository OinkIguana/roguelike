#ifndef __GAME_CELL_H__
#define __GAME_CELL_H__

#include <memory>
#include <vector>
#include "objects/object.h"

namespace Game {
    class Room;

    struct Cell {
        enum class Type { Empty = ' ', Room = '.', Hall = '#', WallH = '-', WallV = '|', Corner = '+', Door = '=' };
        Type type = Type::Empty;
        int x, y;
        int visibility = 0;
        std::shared_ptr<Object> contents;

        Cell(int, int);
        void set_contents(std::shared_ptr<Object> obj);

        void clear();

        bool open(bool = true, bool = true, bool = true) const;
        bool available(bool = true, bool = true, bool = true, bool = true) const;
    };
}

#endif
