#include <memory>
#include "voronota/src/voronotalt.h"
#include "voronota/src/interface.h"

RadicalTessellation from_balls(double probe, const rust::Vec<Ball>& balls, bool with_net) {
    auto periodic_box_corners = rust::Vec<SimplePoint>();
    return RadicalTessellation(balls, periodic_box_corners, probe, with_net);
}

RadicalTessellation from_balls_pbc(double probe, const rust::Vec<Ball>& balls, const rust::Vec<SimplePoint>& periodic_box_corners, bool with_net) {
    return RadicalTessellation(balls, periodic_box_corners, probe, with_net);
}
