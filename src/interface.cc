#include <memory>
#include "voronota/src/voronotalt.h"
#include "voronota/src/interface.h"

RadicalTessellation from_balls(double probe, const rust::Vec<Ball>& balls, bool with_net, const rust::Vec<int>& grouping_of_spheres) {
    auto periodic_box_corners = rust::Vec<SimplePoint>();
    return RadicalTessellation(balls, periodic_box_corners, probe, with_net, grouping_of_spheres);
}

RadicalTessellation from_balls_pbc(double probe, const rust::Vec<Ball>& balls, const rust::Vec<SimplePoint>& periodic_box_corners, bool with_net) {
    const auto empty_grouping = rust::Vec<int>();
    return RadicalTessellation(balls, periodic_box_corners, probe, with_net, empty_grouping);
}
