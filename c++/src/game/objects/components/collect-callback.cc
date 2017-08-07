#include "collect-callback.h"

namespace Game {
    CollectCallback::CollectCallback(std::function<void()> cb) : _cb{cb} {}
    bool CollectCallback::collectable(Object& o, const Object&) const { return o.type == Object::Type::Player; }
    void CollectCallback::collect(Object&, Object&) { _cb(); }
}
