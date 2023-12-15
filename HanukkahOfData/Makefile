data.sqlite : 
	curl https://hanukkah.bluebird.sh/5784/noahs-sqlite.zip > data.zip
	unzip -P 5777 data.zip
	mv 5784/noahs.sqlite $@
	rm -rf 5784 data.zip

speedrundata.sqlite :
	curl https://hanukkah.bluebird.sh/5784-speedrun/noahs-5784-speedrun-sqlite.zip > data.zip
	unzip -P 5777 data.zip
	mv 5784-speedrun/noahs.sqlite $@
	rm -rf 5784-speedrun data.zip
