#pragma once

#include <memory>
#include "rust/cxx.h"

struct RadicalTessellation;
struct Ball;

RadicalTessellation from_balls(double probe, const rust::Vec<Ball>& balls);
