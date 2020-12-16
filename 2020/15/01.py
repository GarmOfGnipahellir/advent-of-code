inp = "18,8,0,5,4,1,20"
inp = [int(n) for n in inp.split(",")]

spoken = {}
last_spoken = 0

for i, n in enumerate(inp):
    spoken[n] = [i+1]
    last_spoken = n

for i in range(len(inp), 2020):
    speak = 0
    if last_spoken in spoken.keys():
        history = spoken[last_spoken]
        num = len(history)
        if num > 1:
            speak = i - history[num-2]
    
    if speak not in spoken.keys():
        spoken[speak] = [i+1]
    else:
        spoken[speak].append(i+1)
    last_spoken = speak
print(spoken)
print(last_spoken)