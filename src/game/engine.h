#ifndef __GAME_ENGINE_H__
#define __GAME_ENGINE_H__

#include <memory>

namespace Game {
    class View;
    class Map;
    struct Info;

    class Engine {
        std::shared_ptr<View> view;
        std::shared_ptr<Info> info;
        std::shared_ptr<Map> map;
    public:
        Engine(std::shared_ptr<View>);
        int start();
    };
}

#endif
