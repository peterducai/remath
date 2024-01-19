/**
 * Visualization of circles and trigonometric functions
 *  PROCESSING?
 * Based on http://math.stackexchange.com/a/734790
**/

// Settings
var speed = 1.5;
var backgroundColor = color(250, 248, 235);
var inkColor = color(84, 81, 43);
var triangleColor = color(240, 210, 57);
var middleLinesColor = color(red(inkColor), green(inkColor), blue(inkColor), 50);

// Globals
var theta = 0; // In degrees
var radius = 50;
var diameter = 2 * radius;
textFont(createFont("serif", 10));

// Draw an arrowhead (for graph axes)
var drawArrow = function(x, y, dir) {
    pushMatrix();
    translate(x, y);
    rotate(dir);
    fill(inkColor);
    noStroke();
    triangle(0, 0, -3, 8, 3, 8);
    popMatrix();
};

// Draw a pair of graphs
var drawGraphPair = function(axis, rotation, func, funcName) {
    pushMatrix();
    rotate(rotation);

    // Labels
    fill(inkColor);
    text(axis, 75, -radius-5);
    text('θ', 192, 0);
    text(axis + ' = ' + funcName + '(θ)', 113, -radius-5);

    // Main graph
    fill(backgroundColor);
    stroke(inkColor);
    beginShape();
    for (var i=70; i<190; i++) {
        vertex(i, func(theta+(i-70)*2)*radius);
    }
    endShape();
    line(70, 0, 190, 0);
    line(70, radius, 70, -radius);
    drawArrow(190, 0, 90);
    drawArrow(70, -radius, 0);

    // Secondary graph
    fill(backgroundColor);
    stroke(inkColor);
    beginShape();
    for (var i=-190; i<-70; i++) {
        vertex(i, func(theta+(i+70)*2)*radius);
    }
    endShape();
    line(-70, 0, -190, 0);
    line(-70, radius, -70, -radius);
    drawArrow(-70, -radius, 0);

    popMatrix();
};

// Draw the animation
var draw = function() {
    // Update
    theta = theta+speed % 360;
    var x = cos(theta+180);
    var y = sin(theta+180);

    // Begin drawing everything
    background(backgroundColor);
    translate(100, 100);

    // Circle
    fill(backgroundColor);
    stroke(inkColor);
    ellipse(0, 0, diameter, diameter);
    
    // Moving lines
    stroke(middleLinesColor);
    line(x*radius, -70, x*radius, 70);   // Vertical
    line(-70, -y*radius, 70, -y*radius); // Horizontal
    
    // Triangle
    fill(triangleColor);
    stroke(inkColor);
    triangle(0, 0, x*radius, 0, x*radius, -y*radius);
    
    // Graphs
    drawGraphPair('y', 0, sin, 'sin');
    drawGraphPair('x', 90, cos, 'cos');
};