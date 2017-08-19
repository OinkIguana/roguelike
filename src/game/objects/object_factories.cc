#include "object.h"
#include "components/keyboard-input.h"
#include "components/random-movement.h"
#include "components/basic-attackable.h"
#include "components/collect-callback.h"
#include "components/coin-pickup.h"
#include "components/named.h"
#include "components/eyes.h"
#include "components/drops.h"

namespace Game {
    std::shared_ptr<Object> Object::Player() {
        return std::make_shared<Object>(
            Type::Player, '@', Stats{ 100, 100, 25, 25 },
            std::vector<ObjectComponent*>{ new KeyboardInput, new Named{"You"}, new Eyes }
        );
    }

    std::shared_ptr<Object> Object::Goblin() {
        return std::make_shared<Object>(
            Type::Goblin, 'G', Stats{ 100, 100, 5, 2 },
            std::vector<ObjectComponent*>{ new BasicAttackable, new RandomMovement, new Named{"Goblin"}, new Drops{ Object::Gold(3) } }
        );
    }

    std::shared_ptr<Object> Object::Gold(int value) {
        return std::make_shared<Object>(
            Type::Gold, '$', Stats{},
            std::vector<ObjectComponent*>{ new CoinPickup{value}, new Named{"Gold"} }
        );
    }

    std::shared_ptr<Object> Object::Exit(std::function<void()> cb) {
        return std::make_shared<Object>(
            Type::Exit, '\\', Stats{ 100, 100, 0, 0 },
            std::vector<ObjectComponent*>{ new CollectCallback{cb} }
        );
    }
}
