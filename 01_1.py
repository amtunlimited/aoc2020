ledger = set()

for line in open("01.in"):
    number = int(line)
    if (2020-number) in ledger:
        print(number * (2020-number))
        break
    ledger.add(number)
