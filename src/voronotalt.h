#pragma once

#include <stdexcept>
#include "interface.h"
#include "rust/cxx.h"
#include "voronota/src/voronotalt/voronotalt.h"
#include "voronotalt/periodic_box.h"
#include <iostream>

struct SimplePoint
{
	SimplePoint() : x(0.0), y(0.0), z(0.0)
	{
	}

	SimplePoint(double x, double y, double z) : x(x), y(y), z(z)
	{
	}

	double x;
	double y;
	double z;
};

struct Ball
{
	Ball() : x(0.0), y(0.0), z(0.0), r(0.0)
	{
	}

	Ball(double x, double y, double z, double r) : x(x), y(y), z(z), r(r)
	{
	}

	double x;
	double y;
	double z;
	double r;
};

struct Contact
{
	Contact() : index_a(0), index_b(0), area(0.0), arc_length(0.0)
	{
	}

	int index_a;
	int index_b;
	double area;
	double arc_length;
};

struct Cell
{
	Cell() : sas_area(0.0), volume(0.0), included(false)
	{
	}

	double sas_area;
	double volume;
	bool included;
};

struct TessellationVertex
{
	std::size_t ids_of_spheres[4];
	SimplePoint position;
	double dist_min;
	double dist_max;
};

struct RadicalTessellation
{
	double probe;
	rust::Vec<SimplePoint> periodic_box_corners;
	rust::Vec<Ball> balls;
	rust::Vec<Contact> contacts;
	rust::Vec<Cell> cells;
	rust::Vec<TessellationVertex> vertices;
	bool with_tessellation_net = false;

	RadicalTessellation() : probe(1.4), with_tessellation_net(false)
	{
	}

	RadicalTessellation(const rust::Vec<Ball>& balls, const rust::Vec<SimplePoint>& periodic_box_corners, double probe, bool with_net) : probe(probe), periodic_box_corners(periodic_box_corners), balls(balls), with_tessellation_net(with_net)
	{
		recompute(probe);
	}

	int recompute(const double new_probe)
	{
		probe=new_probe;
		contacts.clear();
		cells.clear();

		if(balls.empty())
		{
			return 0;
		}

		voronotalt::RadicalTessellation::Result result;
		const auto spheres = voronotalt::get_spheres_from_balls(balls, probe);
		if (periodic_box_corners.empty()) {
		    const auto disabled_periodic_box = voronotalt::PeriodicBox();
			voronotalt::RadicalTessellation::construct_full_tessellation(spheres, disabled_periodic_box, with_tessellation_net, result);
		} else {
			if (periodic_box_corners.size() != 2) {
				throw std::runtime_error("Invalid periodic box corners");
			}
			std::vector<voronotalt::SimplePoint> corners(2);
			for (std::size_t i = 0; i < corners.size(); i++) {
				corners[i].x = periodic_box_corners[i].x;
				corners[i].y = periodic_box_corners[i].y;
				corners[i].z = periodic_box_corners[i].z;
			}
			const auto periodic_box = voronotalt::PeriodicBox::create_periodic_box_from_corners(corners);
			voronotalt::RadicalTessellation::construct_full_tessellation(spheres, periodic_box, with_tessellation_net, result);
		}

		if(result.contacts_summaries.empty() || result.cells_summaries.empty())
		{
			return 0;
		}

		contacts.reserve(result.contacts_summaries.size());
		for (auto &summary : result.contacts_summaries)
		{
			Contact contact;
			contact.index_a=summary.id_a;
			contact.index_b=summary.id_b;
			contact.area=summary.area;
			contact.arc_length=summary.arc_length;
			contacts.emplace_back(contact);
		}

		std::vector<Cell> temp_cells(balls.size());
		for(auto &summary : result.cells_summaries)
		{
			auto index=static_cast<std::size_t>(summary.id);
			temp_cells.at(index).sas_area=summary.sas_area;
			temp_cells.at(index).volume=summary.sas_inside_volume;
			temp_cells.at(index).included=true;
		}
		cells.reserve(temp_cells.size());
		std::copy(temp_cells.begin(), temp_cells.end(), std::back_inserter(cells));

		if (with_tessellation_net) {
		    std::cout << "Num vertices = " << result.tessellation_net.tes_vertices.size() << std::endl;
		    vertices.reserve(result.tessellation_net.tes_vertices.size());
			for (const auto &vertex : result.tessellation_net.tes_vertices) {
			    TessellationVertex tessellation_vertex;
                tessellation_vertex.ids_of_spheres[0] = vertex.ids_of_spheres[0];
                tessellation_vertex.ids_of_spheres[1] = vertex.ids_of_spheres[1];
                tessellation_vertex.ids_of_spheres[2] = vertex.ids_of_spheres[2];
                tessellation_vertex.ids_of_spheres[3] = vertex.ids_of_spheres[3];
                tessellation_vertex.position.x = vertex.position.x;
                tessellation_vertex.position.y = vertex.position.y;
                tessellation_vertex.position.z = vertex.position.z;
                tessellation_vertex.dist_min = vertex.dist_min;
                tessellation_vertex.dist_max = vertex.dist_max;
                vertices.emplace_back(tessellation_vertex);
			}
        }

		return static_cast<int>(contacts.size());
	}
};
