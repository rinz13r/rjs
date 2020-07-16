function f (x, y) {
    this.x = x;
    this.y = y;
}

var p = new f (1, 2);
print (p.x);
print (p.y);
