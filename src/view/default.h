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

        std::function<Game::Command()> state = [this] () { return character_select(); };
        std::string map;
    public:
        Default();
        ~Default();
        virtual void update(Game::Update) override;
        virtual Game::Command command() override;
        virtual void redraw() override;
    };
}

#endif
