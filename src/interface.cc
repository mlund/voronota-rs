#include <memory>
#include "voronota/src/voronotalt.h"
#include "voronota/src/interface.h"

RadicalTessellation from_balls(double probe, const rust::Vec<Ball>& balls) {
    auto periodic_box_corners = rust::Vec<SimplePoint>();
    return RadicalTessellation(balls, periodic_box_corners, probe);
}
