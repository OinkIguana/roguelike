#ifndef __VIEW_DEFAULT_H__
#define __VIEW_DEFAULT_H__

#include <functional>
#include "../game/view.h"
#include "../game/command.h"
#include "../game/update.h"

namespace View {
    class Default : public Game::View {
        Game::Command character_select();
        Game::Command world_view();

        Game::Direction last_direction;

        std::function<Game::Command()> state = [this] () { return character_select(); };

        struct { int x, y, w, h; } viewport{ 0, 0, 70, 25 };

        std::vector<std::string> map;
        std::vector<std::string> objects;
        std::string names[2];
        std::string message = "Nothing has happened.";
        int level = 0;
        Game::Stats stats[2] { {}, {0, 0, 0, 0} };
        Game::Inventory inventory;
    public:
        Default();
        ~Default();
        virtual void update(Game::Update) override;
        virtual Game::Command command() override;
        virtual void redraw() override;
    };
}

#endif
