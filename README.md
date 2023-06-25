# Maps server

This is a project that provides the possibility to store your collections of maps information (Point in the map, tracks or geometries)


## Development

A dev container is the way to go with this. It uses docker compose to spin up both the dev container and the postgis database container

### Problems?

Here are a bunch of possible problems that can be found when developing for this project

#### type "geometry" does not exist

You need to run the following command within the postgres db to enable the postgis extension
```
CREATE EXTENSION postgis;
```
