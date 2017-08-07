#ifndef __GAME_ENGINE_H__
#define __GAME_ENGINE_H__

#include <memory>

namespace Game {
    class View;
    class Map;
    class Player;
    class Object;
    struct Info;
    struct Command;

    class Engine {
        std::shared_ptr<View> view;
        std::shared_ptr<Info> info;
        std::shared_ptr<Map> map;

        std::shared_ptr<Object> player;
        bool proc(Command, std::shared_ptr<Object>);
    public:
        Engine(std::shared_ptr<View>);
        int start();
    };
}

#endif
