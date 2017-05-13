#include "../command.h"
#include "component.h"

namespace Game {
    ObjectComponent::~ObjectComponent() {}
    std::string ObjectComponent::name() const { return ""; }
    void ObjectComponent::collect(Object&, Object&) {}
    bool ObjectComponent::collectable(Object&, const Object&) const { return false; }
    void ObjectComponent::interact(Object&, Object&) {}
    bool ObjectComponent::interactable(Object&, const Object&) const { return false; }
    void ObjectComponent::attack(Object&, Object&) { }
    bool ObjectComponent::attackable(Object&, const Object&) const { return false; }
    Command ObjectComponent::update(Command, Object&) { return Command{ CommandType::Idle }; }
}
