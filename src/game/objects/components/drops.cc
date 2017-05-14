#include "drops.h"

namespace Game {
    Drops::Drops(std::shared_ptr<Object> drop) : _drop{drop} {}
    void Drops::on_destroy(Object& me, Cell& cell) {
        cell.set_contents(_drop);
    }
}
