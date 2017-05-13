#include "collect-callback.h"

namespace Game {
    CollectCallback::CollectCallback(std::function<void()> cb) : _cb{cb} {}
    bool CollectCallback::collectable(Object&, const Object&) const { return true; }
    void CollectCallback::collect(Object&, Object&) { _cb(); }
}
