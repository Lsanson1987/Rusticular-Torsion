import csv

##need to fix soon possible error with file name ?????/
with open("correct_monster_data.csv", 'w') as csvfile:
    fields = ['Name', 'AC', 'HP', 'Int', 'Notes']
    writer = csv.DictWriter(csvfile, fieldnames=fields) 
    writer.writeheader()
    writer.writerow({'Name' : 'a', 'AC' : 'a', 'HP' : 'a', 'Int' : 'a', 'Notes' : 'a'})
    with open("Monster_Data.txt", encoding="utf-8") as f:
        for line in f:
            clean_line = line.strip()[1:-2]
            array = clean_line.split(", ")
            name = array[2][7:-1]
            ac = array[3][4:]
            hp = array[4][4:]
            dex = array[5][5:]
            print(dex)
            writer.writerow({'Name' : name, 'AC' : ac, 'HP' : hp, 'Int' : dex, 'Notes' : ''})

