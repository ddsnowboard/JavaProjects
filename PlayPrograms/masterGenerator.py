#!/usr/bin/python3
from os import listdir
from sys import argv
from re import compile
from pprint import pprint
AUDIO_FORMATS = [".wav", ".mp3"]
OUTPUT_NAME = "MasterCueList.txt"
def clean(l):
    while l[0] != "T":
        l = l[1:]
    return l
def sort_function(l):
    format = compile("Track (?P<number>[0-9]{1,2})")
    return int(format.search(l).group("number"))
if "-f" in argv or not OUTPUT_NAME in listdir():
    l = [i for i in listdir() if not i[:2] == "._"]
    files = []
    for f in AUDIO_FORMATS:
        files += [i[:-4] for i in l if i[-4:] == f]
    # for p in range(len(files)):
    #     files[p] = clean(files[p])
    # files.sort(key=sort_function)
    files.sort()
    with open(OUTPUT_NAME, 'w') as w:
        w.write(": \n".join(files))
else:
    print("Use masterGenerator.py -f to force a new file")
