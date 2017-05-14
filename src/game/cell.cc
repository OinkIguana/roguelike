#include "cell.h"

namespace Game {
    Cell::Cell(int x, int y) : x{ x }, y{ y } {}

    void Cell::set_contents(std::shared_ptr<Object> obj) {
        obj->x = x;
        obj->y = y;
        contents = obj;
        contents->cell(shared_from_this());
    }

    bool Cell::open(bool rooms, bool halls, bool doors) const {
        return (rooms && type == Cell::Type::Room) || (halls && type == Cell::Type::Hall) || (doors && type == Cell::Type::Door);
    }

    bool Cell::empty(bool rooms, bool halls, bool doors) const {
        return !contents && ((rooms && type == Cell::Type::Room) || (halls && type == Cell::Type::Hall) || (doors && type == Cell::Type::Door));
    }

    bool Cell::available(Object& who, bool rooms, bool halls, bool doors, bool collect) const {
        return open(rooms, halls, doors) && (!contents || (collect && contents->collectable(who)));
    }

    void Cell::clear() {
        contents->x = -1;
        contents->y = -1;
        contents = nullptr;
    }
}
