#include "room.h"
#include "cell.h"
#include <random>

namespace Game {
    std::default_random_engine rng;
    Room::Room(std::vector<std::shared_ptr<Cell>> cells) : cells{ cells } {}

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
        } while(!cl->available(true, false, false, false));
        return cl;
    }
}
