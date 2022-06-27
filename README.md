# 3d-grapher
A GUI tool to graph position vectors in 3D. Wirtten in Rust.


#required applications

This project used GNUplot to plot the data, so you will need to install that
or the graph will not appear.

//TODO: add install instructions

# references 

 article: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

 
 I'd like to thank maklad for their Pratt parser adapted in Rust. It converted strings into S- expressions and I thought that would make parsing strings and doing math with them easier.

The repo is at https://github.com/matklad/minipratt.


#notes

This projects is in a very early stage, so somethings are prone to not work, and some stuff isnt supported (yet).

Current issues that will be fixed later in development:
console gets flooded with some of the inputed equation data when the graph is created (I have yet to figure out why)
Trig functions will not graph
only supports 1 digit numbers, 
