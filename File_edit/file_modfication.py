import csv

##need to fix soon possible error with file name ?????/
with open("correct_monster_data.csv", 'w') as csvfile:
    fields = ['id', 'ruleset', 'name', 'AC', 'HP', 'Dex', 'HDP']
    writer = csv.DictWriter(csvfile, fieldnames=fields) 
    writer.writeheader()
    with open("Monster_Data.txt", 'r') as readfile : 
        ##issue with this
        lines = readfile.readline()
        for line in lines:
            ##remove last },
            clean_line = line.strip()[1:-2]
            ##fix this up
            clean_line_list = clean_line.find(", ")
            data_row = []
            ##fix this as well small stuff
            for var in clean_line_list :
                index = var.index(":")
                value = var[index + 1:].strip().replace("'", ' ').replace('"', ' ')
                data_row.append(value)
            writer.writerow({'id' : data_row[0], 'ruleset' : list[1], 'name' : list[2], 'AC' : list[3], 'HP' : list[3], 'Dex' : list[4], 'HDP' : list[5]})

