# Stjernekikking

På Nordpolen er det naturleg nok lite lysforureining, noko som gjer at mange alvar har stjernekikking som hobby. Dei har til dette rigga opp ein felles stjernekikkert. Ei natt det er ekstra stjerneklart kikar Elfias lenge på himmelen, representert i [denne fila](src/stars.txt).

Stjernekikkerten sit synsfelt ser slik ut, med `@` som midtpunkt:
```
    #####  
  #########
 #####@#####
  #########
    #####
```

## Oppgåve
Elfias flyttar midtpunktet av kikkerten sakte rundt om kring på himmelen medan han observerar i kikkerten. Han følger punkta på [denne lista](src/path.txt). Kikkerten flyttar seg anten horisontalt eller vertikalt, og alltid etter kortaste veg til neste punkt. Samstundes flyttar stjernene seg over nattehimmelen, frå høgre mot venstre. Stjernene flyttar seg 1 himmelenhet mot venstre for kvar enhet kikkerten flyttar seg. Stjernene som går ut av syne til venstre dukkar sjølvsagt opp på høgre side. Kor mange unike stjerner observerar Elfias i kikkerten sit synsfelt denne natta?

## Presiseringar
- Inputfila er på format `x-posisjon, y-posisjon`
- Koordinatane er 0-indekserte, så øverst til venstre er `0, 0`.
- Som vanleg går er x-aksen fra venstre mot høgre, og y-aksen nedover i fila.
- Elfias tel stjernene han har i kikkerten heile vegen, ikkje berre på koordinatane frå lista.
- Elfias tel stjernene først, så flyttar han på kikkerten.
- Basen kikkerten står på roterar ikkje 360 grader, så sentrum på kikkerten kan ikkje gå utanom grensene for himmel-koordinatane.
