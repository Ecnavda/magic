def main():

    output = []
    
    with open("mtg_card_sets.txt", 'r') as card_sets:
        
        for line in card_sets.readlines():
            name, release = line.split(";")
            release = release.strip()
            print("INSERT INTO card_sets (name, release) VALUES (\"%s\", \"%s\");" % (name, release))
        
        # = [line.split(";") for line in card_sets.readlines()]
        #map(lambda x, y: print("INSERT INTO card_sets (name, release) VALUES ('%s', '%s')" % (x, y.strip()), [line.split(";") for line in card_sets.readlines()]))


if __name__ =="__main__":
    main()