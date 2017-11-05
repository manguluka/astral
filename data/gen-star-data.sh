#!/bin/sh
#Create subset of stars (those with common names) from main dataset
xsv search -i -s proper '[.*\S.*]' hygdata_v3.csv | xsv select proper,x,y,z,dist,ra,dec > star_data.csv