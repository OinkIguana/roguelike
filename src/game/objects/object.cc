#include "object.h"
#include "../command.h"

namespace Game {
    void Object::collect(std::shared_ptr<Object>) {}
    bool Object::collectable(std::shared_ptr<Object>) const { return false; }
    void Object::interact(std::shared_ptr<Object>) {}
    bool Object::interactable(std::shared_ptr<Object>) const { return false; }
    void Object::attack(std::shared_ptr<Object>) {}
    bool Object::attackable(std::shared_ptr<Object>) const { return false; }
    Command Object::update(Command) { return Command{ CommandType::Idle }; }

    std::shared_ptr<Cell> Object::cell() const {
        return _cell.lock();
    }
    void Object::cell(std::shared_ptr<Cell> ncell) {
        _cell = ncell;
    }
}
