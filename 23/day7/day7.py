
with open('./day7.txt', 'r') as file:
    content = file.read()


content = [x.split(" ") for x in content.split('\n')]

priority = {
    "2": 1,
    "3": 2,
    "4": 3,
    "5": 4,
    "6": 5,
    "7": 6,
    "8": 7,
    "9": 8,
    "T": 9,
    "J": 10,
    "Q": 11,
    "K": 12,
    "A": 13
}

sorted_cards = []
winnings = 0


def sort_cards(a):
    for i in range(0, len(a) - 1):
        for k in range(0, len(a) - i - 1):
            for j in range(0, 5):
                if priority[a[k][j]] > priority[a[k+1][j]]:
                    a[k], a[k+1] = a[k+1], a[k]
                    break
                elif priority[a[k][j]] == priority[a[k+1][j]]:
                    j += 1
                else:
                    break
    return a

for card in content:
    card_set = set()
    for char in card[0]:
        card_set.add(char)
    print(len(card_set))
    if len(card_set) != 2 and len(card_set) != 3:
        sorted_cards.append([card[0], len(card_set)])
    elif len(card_set) == 2:
        for char in card[0]:
            if card[0].count(char) == 4:
                sorted_cards.append([card[0], 'fourkind'])
                break
            elif card[0].count(char) == 3:
                sorted_cards.append([card[0], 'fullhouse'])
                break
    else:
        for char in card[0]:
            if card[0].count(char) == 3:
                sorted_cards.append([card[0], 'threekind'])
                break
            elif card[0].count(char) == 2:
                sorted_cards.append([card[0], 'twopair'])
                break

fivekind = sort_cards(
    [x[0] for x in list(filter(lambda x: x[1] == 1, sorted_cards))])
fourkind = sort_cards([x[0] for x in list(
    filter(lambda x: x[1] == 'fourkind', sorted_cards))])
fullhouse = sort_cards([x[0] for x in list(
    filter(lambda x: x[1] == 'fullhouse', sorted_cards))])
threekind = sort_cards([x[0] for x in list(
    filter(lambda x: x[1] == 'threekind', sorted_cards))])
twopair = sort_cards([x[0]
                      for x in list(filter(lambda x: x[1] == 'twopair', sorted_cards))])
onepair = sort_cards(
    [x[0] for x in list(filter(lambda x: x[1] == 4, sorted_cards))])
high = sort_cards([x[0]
                  for x in list(filter(lambda x: x[1] == 5, sorted_cards))])

high.extend(onepair+twopair+threekind+fullhouse+fourkind+fivekind)

print(content)
print(high)

for i in range(len(high)):
    bid = next(x[1] for x in content if x[0] == high[i])
    winnings += (i+1) * int(bid)

print(winnings)
