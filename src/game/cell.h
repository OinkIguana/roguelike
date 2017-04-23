#ifndef __GAME_CELL_H__
#define __GAME_CELL_H__

#include <memory>
#include <vector>

namespace Game {
    class Room;

    struct Cell {
        enum class Type { Empty = ' ', Room = '.', Hall = '#', WallH = '-', WallV = '|', Door = '+' };
        Type type = Type::Empty;
        int x, y;
        int visibility = 0;

        Cell(int, int);
    };
}

#endif
