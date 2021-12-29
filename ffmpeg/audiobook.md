# Techniques for manipulating audiobook files

Concat a list of .mp3 files

    # format list of mp3 files for ffmpeg processing:
    #    file '1.mp3'
    #    file '2.mp3'
    #    file '3.mp3'
    #    ...

    ls *.mp3 | sed "s/\(.*\)/file '\1'/g" > file_list.txt

    # concat files from list
    ffmpeg -f concat -safe 0 -i file_list.txt -c copy output.mp3

Detect silence intervals <= -30dB lasting >= 1 second.

    ffmpeg -i input.mp3 -af silencedetect=n=-30dB:d=1 -f null - 2>&1 | \
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

Split mp3 file based on split points in bash variable:

    ffmpeg -v warning -i input.mp3  -c copy -map 0 -f segment \
        -segment_times "$SPLITPOINTS" "%02d.mp3"
