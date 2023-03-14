#https://tsujimotter.hatenablog.com/entry/pi-approximation-formula

from decimal import *

getcontext().prec = 35

print((Decimal('640320')**3+744).ln()/Decimal('163').sqrt())

## python pi_ln.py
#3.1415926535897932384626433832797267
## pi 35
#3.1415926535897932384626433832795028
#  123456789012345678901234567890
