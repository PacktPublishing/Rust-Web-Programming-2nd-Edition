

# slow method 
data = ["one", "two", "three", "four"]

string = ""

for i in data:
    string += i
    
# faster method
"".join(data)
