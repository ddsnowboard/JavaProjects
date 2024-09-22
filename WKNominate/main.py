import numpy as np
import pandas as pd

datafile = "H118_votes.csv"

def load_and_pivot_data():
    """
    This loads the data from the file and pivots it such that
    the first entry in each row is the ICPSR number of the member and the subsequent entries are
    their votes (1 for yea, 0 for nay, 0.5 for other) on each roll call vote in that congres
    """
    base = pd.read_csv(datafile, usecols=["rollnumber", "icpsr", "cast_code"])
    vote_code_mapping = {
        1: 1.0,
        2: 1.0,
        3: 1.0,
        4: 0.0,
        5: 0.0,
        6: 0.0
    }
    base["vote_code"] = np.vectorize(lambda x: vote_code_mapping.get(x, 0.5))(base["cast_code"])
    members = np.unique(base["icpsr"])
    votes = np.unique(base["rollnumber"])
    def make_row_for_member(member_number):
        l = [base.loc[(base["icpsr"] == member_number) & (base["rollnumber"] == vote_number), "cast_code"] for vote_number in votes]
        return l

    # I need to figure out how to get a list, sorted by rollnumber, of each vote for each member
    grouped = base.groupby("icpsr")[["rollnumber", "vote_code"]]
    print(grouped.head())


if __name__ == "__main__":
    load_and_pivot_data()
