#include <algorithm>
#include "object.h"
#include "../command.h"
#include "component.h"

namespace Game {
    Object::Object(Type type, char symbol, Stats && stats, std::vector<ObjectComponent*> && components) : _components{components}, stats{stats}, symbol{symbol}, type{type} {}

    Object::~Object() {
        std::for_each(_components.begin(), _components.end(), [] (ObjectComponent* c) { delete c; });
    }
    
    std::string Object::name() const {
        std::string name = "";
        for(auto c : _components) {
            const auto cname = c->name();
            if(cname != "") {
                if(name != "") { name += " "; }
                name += cname;
            }
        }
        return name;
    }

    bool Object::dead() const { return _dead; }

    void Object::collect(Object& o) {
        std::for_each(_components.begin(), _components.end(), [&o, this] (ObjectComponent* c) { c->collect(o, *this); });
    }
    bool Object::collectable(Object& o) const {
        return std::any_of(_components.begin(), _components.end(), [&o, this] (const ObjectComponent* c) { return c->collectable(o, *this); });
    }
    void Object::interact(Object& o) {
        std::for_each(_components.begin(), _components.end(), [&o, this] (ObjectComponent* c) { c->interact(o, *this); });
    }
    bool Object::interactable(Object& o) const {
        return std::any_of(_components.begin(), _components.end(), [&o, this] (const ObjectComponent* c) { return c->interactable(o, *this); });
    }
    void Object::attack(Object& o) {
        std::for_each(_components.begin(), _components.end(), [&o, this] (ObjectComponent* c) { c->attack(o, *this); });
    }
    bool Object::attackable(Object& o) const {
        return std::any_of(_components.begin(), _components.end(), [&o, this] (const ObjectComponent* c) { return c->attackable(o, *this); });
    }
    std::vector<Command> Object::update(Command cmd) {
        std::vector<Command> cmds(_components.size());
        std::transform(_components.begin(), _components.end(), cmds.begin(), [&cmd, this] (ObjectComponent* c) { return c->update(cmd, *this); } );
        return cmds;
    }

    std::shared_ptr<Cell> Object::cell() const {
        return _cell.lock();
    }
    void Object::cell(std::shared_ptr<Cell> ncell) {
        _cell = ncell;
    }

    void Object::destroy() {
        auto cl = cell();
        cl->clear();
        _dead = true;
        std::for_each(_components.begin(), _components.end(), [this, cl] (ObjectComponent* c) { c->on_destroy(*this, *cl); });
    }
}
