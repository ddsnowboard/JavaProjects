from tkinter import Tk
a=b=c=0
for a in range(999):
	for b in range(1000-a):
		c = 1000-(a+b)
		if (a**2 + b**2) == c**2 and a<b<c:
			t = Tk()
			t.withdraw()
			t.clipboard_clear()
			t.clipboard_append(str(a*b*c))
			t.destroy()