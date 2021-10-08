export CONCURRENT=$1
export REQUESTS=$2
export TYPE=$3

# From the type of the test work out the Title
TITLE=""
if [[ $TYPE == "single_threaded" ]]; then
  TITLE="Single Threaded Server"
fi
if [[ $TYPE == "single_handler" ]]; then
  TITLE="Concurrent Server with A Single Handler"
fi
if [[ $TYPE == "separate_handler" ]]; then
  TITLE="Concurrent Server with Separate Handlers"
fi

# Work out the different file names
name="$(printf "C%dR%dT%s" $CONCURRENT $REQUESTS $TYPE)"
if [[ -e "img/$name.png" || -L "img/$name.png" ]] ; then
    i=0
    while [[ -e "img/$name-$i.png" || -L "img/$name-$i.png" ]] ; do
        let i++
    done
    name=$name-$i
fi

GNUFILE="gnu/$name.gnuplot"
PNGFILE="img/$name.png"
CSVFILE="csv/$name.csv"
FLAMEFILE="flame/$name.svg"

echo "----------------------------------------------------"
echo "TEST: Concurrent=${CONCURRENT}, Requests=${REQUESTS}"
echo "----------------------------------------------------"

echo "---> Benchmarking Server"
sudo ab \
     -n $REQUESTS \
     -c $CONCURRENT \
     -g $GNUFILE \
     -e $CSVFILE \
     http://localhost:8080/add/1

echo "---> Generating Graph"
gnuplot \
    -e "outputfile='$PNGFILE'" \
    -e "concurrent='$CONCURRENT'" \
    -e "requests='$REQUESTS'" \
    -e "datafile='$GNUFILE'" \
    -e "title='$TITLE'" \
    plot.p

read -p "---> Rename Flamegraph" confirm
mv "flame/$TYPE.svg" $FLAMEFILE
