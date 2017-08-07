#ifndef __GAME_VIEW_H__
#define __GAME_VIEW_H__

namespace Game {
    struct Command;
    struct Update;

    class View {
    public:
        virtual void update(Game::Update) = 0;
        virtual Command command() = 0;
        virtual void redraw() = 0;
    };
}

#endif
