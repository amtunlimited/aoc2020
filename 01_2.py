ledger = set()

for line in open("01.in"):
    number = int(line)
    ledger.add(number)

for x in ledger:
    for y in ledger: 
        if (2020 - x - y) in ledger:
            print(x * y * (2020-x-y))
