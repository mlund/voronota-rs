#include <memory>
#include "voronota/src/voronotalt.h"
#include "voronota/src/interface.h"

RadicalTessellation from_balls(double probe, const rust::Vec<Ball>& balls) {
    return RadicalTessellation(balls, probe);
}
