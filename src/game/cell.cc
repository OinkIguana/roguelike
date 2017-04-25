#include "cell.h"

namespace Game {
    Cell::Cell(int x, int y) : x{ x }, y{ y } {}

    void Cell::set_contents(std::shared_ptr<Object> obj) {
        obj->x = x;
        obj->y = y;
        contents = obj;
    }

    bool Cell::open(bool rooms, bool halls, bool doors) const {
        return (type == Cell::Type::Room && rooms) || (type == Cell::Type::Hall && halls) || (type == Cell::Type::Door && doors);
    }

    bool Cell::available(bool rooms, bool halls, bool doors, bool collect) const {
        return open(rooms, halls, doors) && (!contents || (collect && contents->collectable()));
    }

    void Cell::clear() {
        contents->x = -1;
        contents->y = -1;
        contents = nullptr;
    }
}
