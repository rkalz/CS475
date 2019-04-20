import json

def get_dates():
    rows = []
    with open("dates.csv", "rt", encoding="utf16") as f:
        for row in f:
            rows.append(row.split())
    rows.pop(0)

    fixed_rows = []
    for row in rows:
        fixed_rows.append({
            'date': ' '.join((row[0],row[1],row[2])),
            'count': int(row[3])
        })

    with open('dates.json', 'w') as f:
        json.dump(fixed_rows, f)

def get_sankey(name):
    rows = []
    with open(name + ".csv", "rt", encoding="utf16") as f:
        for row in f:
            rows.append(row.split())
    rows.pop(0)

    fixed_rows = []
    for row in rows:
        fixed_rows.append({
            "source": row[1],
            "target": row[0].replace('.', ''),
            "value": int(row[2])
        })

    with open(name + '.json', 'w') as f:
        json.dump(fixed_rows, f)

if __name__ == "__main__":
    get_dates()
    get_sankey("likes")
    get_sankey("mentions")