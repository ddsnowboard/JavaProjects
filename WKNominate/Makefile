VOTE_FILE=votes.csv
votes_by_member.csv :  ${VOTE_FILE} group_by_member.py
	python3 group_by_member.py $< > $@

${VOTE_FILE} : ${VOTE_FILE}.xz
	unxz --keep $<

members.csv : members.csv.xz
	unxz --keep $<

ordered_member_data.csv : members.csv ${VOTE_FILE} generate_ordered_member_data.py
	python3 generate_ordered_member_data.py > $@

decompositions.csv : votes_by_member.csv analyze.py
	python3 analyze.py > $@

combined.csv : decompositions.csv ordered_member_data.csv
	python3 combine.py > $@

.database : combined.csv
	sqlite-utils drop-table data.sqlite members --ignore
	sqlite-utils insert -c --sniff -d data.sqlite members combined.csv --silent
	touch .database

data.sqlite : .database

clean : 
	$(RM) ${VOTE_FILE} votes_by_member.csv members.csv ordered_member_data.csv combined.csv data.sqlite .database decompositions.csv
