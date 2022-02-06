# Techniques for manipulating audiobook files

Concat a list of .mp3 files

    ls *.mp3 | sed "s/.*/file '&'/" > concat_list.txt
    ffmpeg -f concat -safe 0 -i concat_list.txt -c copy concat.mp3

Detect silence intervals <= -30dB lasting >= 1 second.

    ffmpeg -i concat.mp3 -af silencedetect=n=-30dB:d=1 -f null - 2>&1 | \
        grep 'silencedetect @' | sed 's/^[^]]*\] //' | paste -d " " - - | \
        sed 's/|//' > silence_intervals.txt

Get split points for the 50 longest silence intervals:

    sort -g --key=6 silence_intervals.txt | tail -50 | sort -g --key=2 | \
        awk '{ print $2 + ($6 / 2); }' > split_points.txt

Get split points on silence intervals >= 180 seconds apart

    sort -g --key=2 silence_intervals.txt | \
        awk -v prev=0 '{ \
            split_point = ($2 + $4)/2; dur = split_point - prev; \
            if (dur >= 180) { print split_point; prev = split_point; } \
        }' > split_points.txt

Capture split points into a comma separated bash variable:

    SPLITPOINTS=$(tr '\n' ',' < split_points.txt | sed 's/,$//')

    ffmpeg -v warning -i concat.mp3  -c copy -map 0 -f segment \
        -segment_times "$SPLITPOINTS" "%02d.mp3"

Read file containing information about chapters, and move files into those directories

    # Input file should look like this:
    #
    # chap00/
    # 00.mp3
    # 01.mp3
    #
    # chap01/
    # 02.mp3
    # 03.mp3
    # 04.mp3
    # ...

    # make chapter directories and move corresponding files inside
    ls | awk '/chap/{ print "mkdir " $1; d=$1; } /mp3/{ print "mv " $1 " " d; }' input_file | sh

