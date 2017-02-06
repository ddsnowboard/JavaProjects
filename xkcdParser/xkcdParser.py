#! /usr/bin/python
from urllib2 import urlopen
from sys import argv
from re import compile
from HTMLParser import HTMLParser

url = "http://www.xkcd.com"
if len(argv) > 1:
    try:
        which = int(argv[1])
        url += "/" + str(which)
    except ValueError:
        pass

page = urlopen(url).read()
pattern = compile(r'<img src="//imgs\.xkcd\.com/comics/.*?" title="(.*?)"')
parser = HTMLParser()
print parser.unescape(pattern.search(page).group(1))
