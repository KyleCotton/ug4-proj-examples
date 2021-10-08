set output outputfile
full_title = sprintf("%s Requests, with %s Concurrent Connections on a\n%s", requests, concurrent, title)
set title full_title

set xlabel "Number of Requests"
set ylabel "Response Time (ms)"

set terminal png

set size ratio 0.6
set grid

plot datafile using 9 smooth sbezier with lines title title
