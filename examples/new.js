function Point (x, y) {
    this.x = x;
    this.y = y;
}
Point.prototype.print = function () {
    print (this.x, this.y);
}
Point.prototype.add = function (other) {
    return new Point (this.x + other.x, this.y + other.y);
}
Point.prototype.toString = function () {
    return String(this.x) + " " + String(this.y);
}

var p1 = new Point (1, 2);
var p2 = new Point (3, 4);
p1.print ();
p2.print ();

p1.add (p2).print ();
