#ifndef __GAME_CELL_H__
#define __GAME_CELL_H__

#include <memory>
#include <vector>
#include "objects/object.h"

namespace Game {
    class Room;
    class Map;

    struct Cell : std::enable_shared_from_this<Cell> {
    public:
        enum class Type { Empty = ' ', Room = '.', Hall = '#', WallH = '-', WallV = '|', Corner = '+', Door = '=' };
    private:
        friend class Room;
        friend class Map;
        std::vector<Room*> rooms;
        Map& map;
        int visible = false;
    public:
        Type type = Type::Empty;
        int x, y;
        std::shared_ptr<Object> contents;

        Cell(int, int, Map&);
        void set_contents(std::shared_ptr<Object> obj);
        void set_visible(bool, bool = true);
        void clear();
        char symbol() const;
        bool open(bool = true, bool = true, bool = true) const;
        bool empty(bool = true, bool = true, bool = true) const;
        bool available(Object&, bool = true, bool = true, bool = true, bool = true) const;
        std::shared_ptr<Cell> north();
        std::shared_ptr<Cell> east();
        std::shared_ptr<Cell> south();
        std::shared_ptr<Cell> west();
    };
}

#endif
