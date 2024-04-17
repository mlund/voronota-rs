#pragma once

#include <memory>
#include "rust/cxx.h"

struct RadicalTessellation;
struct Ball;
struct SimplePoint;

RadicalTessellation from_balls(double probe, const rust::Vec<Ball>& balls);
RadicalTessellation from_balls_pbc(double probe, const rust::Vec<Ball>& balls, const rust::Vec<SimplePoint>& periodic_box_corners);