#include "cell.h"
#include "room.h"
#include "map.h"

namespace Game {
    Cell::Cell(int x, int y, Map& map) : map{map}, x{ x }, y{ y } {}

    char Cell::symbol() const {
        return visible ? static_cast<char>(type) : ' ';
    }

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

    void Cell::set_visible(bool visible, bool whole_room) {
        this->visible = visible;
        if(whole_room) {
            for(auto & room : rooms) room->set_visible(visible);
        }
    }

    std::shared_ptr<Cell> Cell::north() { return map.cell_at(x, y - 1); }
    std::shared_ptr<Cell> Cell::east() { return map.cell_at(x + 1, y); }
    std::shared_ptr<Cell> Cell::south() { return map.cell_at(x, y + 1); }
    std::shared_ptr<Cell> Cell::west() { return map.cell_at(x - 1, y); }
}
