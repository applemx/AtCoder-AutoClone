n=int(input())
l=[input() for _ in range(n)]
s=set()
for i in range(n):
  if l[i] not in s and l[i][::-1] not in s:
    s.add(l[i])
print(len(s))