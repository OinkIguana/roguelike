#ifndef __GAME_OBJECT_H__
#define __GAME_OBJECT_H__

#include <memory>

namespace Game {
    class Cell;

    class Object {
    public:
        virtual char symbol() const = 0;
        virtual bool collectable() const;
        virtual void collect(std::shared_ptr<Object>);
        virtual bool interactable() const;
        virtual void interact(std::shared_ptr<Object>);
        virtual bool attackable() const;
        virtual void attack(std::shared_ptr<Object>);
        int x, y;
    };
}

#endif
