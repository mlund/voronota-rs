#include <memory>
#include "voronota-rs/src/voronotalt.h"
#include "voronota-rs/src/interface.h"

RadicalTessellation from_balls(double probe, const rust::Vec<Ball>& balls) {
    return RadicalTessellation(balls, probe);
}
