sudoku = []

all = []

for i in range(81):
    sudoku.append(i)

for i in sudoku:
    x = i % 9
    y = int(i / 9)
    
    row = []
    for sus in range(9):
        row.append((y * 9) + sus)

    column = []

    for sus in range(9):
        column.append((sus * 9) + x)

    box = []

    xx = x - (x % 3)
    yy = y - (y % 3)

    for sus in range(xx, xx + 3):
        for sis in range(yy, yy + 3):
            box.append(sus + (sis * 9))


    # remove double numbers
    for rownum in row:
        if rownum in column:
            column.remove(rownum)
    for rownum in row:
        if rownum in box:
            box.remove(rownum)
    for colnum in column:
        if colnum in box:
            box.remove(colnum)

    l = []
    l.extend(row)
    l.extend(column)
    l.extend(box)

    all.append(l)
    
# generate rust code
print("let precalc = " + str(all))  