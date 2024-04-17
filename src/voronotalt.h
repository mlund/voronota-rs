#ifndef VORONOTALT_H_
#define VORONOTALT_H_

#include <stdexcept>
#include "rust/cxx.h"
#include "voronota/src/voronotalt/voronotalt.h"
#include <iostream>

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

struct RadicalTessellation
{
	double probe;
	rust::Vec<Ball> balls;
	rust::Vec<Contact> contacts;
	rust::Vec<Cell> cells;

	RadicalTessellation() : probe(1.4)
	{
	}

	RadicalTessellation(const rust::Vec<Ball>& balls, double probe) : probe(probe), balls(balls)
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
		voronotalt::RadicalTessellation::construct_full_tessellation(voronotalt::get_spheres_from_balls(balls, probe), result);

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

		return static_cast<int>(contacts.size());
	}
};

#endif /* VORONOTALT_H_ */
