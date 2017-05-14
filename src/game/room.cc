#include "room.h"
#include "cell.h"
#include "random.h"

namespace Game {
    Room::Room(std::vector<std::shared_ptr<Cell>> cells) : cells{ cells } {
        for(auto& cell : cells) {
            cell->rooms.emplace_back(this);
        }
    }

    bool Room::contains(std::shared_ptr<Object> obj) const {
        for(auto& cell : cells) {
            if(cell->contents == obj) {
                return true;
            }
        }
        return false;
    }

    bool Room::full() const {
        for(auto& cell : cells) {
            if(cell->type == Cell::Type::Room && !cell->contents) {
                return false;
            }
        }
        return true;
    }

    std::shared_ptr<Cell> Room::choose_cell() const {
        std::shared_ptr<Cell> cl;
        std::uniform_int_distribution<int> rc(0, cells.size() - 1);
        do {
            cl = cells[rc(rng)];
        } while(!cl->empty(true, false, false));
        return cl;
    }

    void Room::set_visible(bool visible) {
        for(auto& cell : cells) {
            cell->visible = visible;
        }
    }
}
