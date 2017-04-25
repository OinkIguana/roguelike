#include "object.h"

namespace Game {
    void Object::collect(std::shared_ptr<Object>) {}
    bool Object::collectable() const { return false; }
    void Object::interact(std::shared_ptr<Object>) {}
    bool Object::interactable() const { return false; }
    void Object::attack(std::shared_ptr<Object>) {}
    bool Object::attackable() const { return false; }
}
