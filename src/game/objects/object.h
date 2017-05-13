#ifndef __GAME_OBJECT_H__
#define __GAME_OBJECT_H__

#include <memory>
#include <vector>
#include "stats.h"

namespace Game {
    class Cell;
    class ObjectComponent;
    struct Command;

    class Object {
    public:
        enum class Type { Player, Goblin, Exit };
    private:
        std::weak_ptr<Cell> _cell;
        std::vector<ObjectComponent*> _components;
    public:
        Object(Type, char, Stats &&, std::vector<ObjectComponent*> &&);
        ~Object();
        std::shared_ptr<Cell> cell() const;
        void cell(std::shared_ptr<Cell>);
        std::string name() const;
        bool collectable(Object&) const;
        void collect(Object&);
        bool interactable(Object&) const;
        void interact(Object&);
        bool attackable(Object&) const;
        void attack(Object&);
        std::vector<Command> update(Command);

        Stats stats;
        int x, y;
        const char symbol;
        const Type type;

        // factories
        static std::shared_ptr<Object> Player();
        static std::shared_ptr<Object> Goblin();
        static std::shared_ptr<Object> Exit(std::function<void()>);
    };
}

#endif
