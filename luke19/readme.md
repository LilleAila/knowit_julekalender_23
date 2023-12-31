# Det beste juletreet

Alvhild og Halvor henger opp julekuler på juletreet. Halvor plukkar kulene ut av eska og rekker dei til Alvhild, som henger dei opp. Alvhild byrjar på toppen av treet, og jobbar seg nedover. I eit forsøk på å fordele fargane på kulene på eit litt interessant vis bestemmer dei seg for å henge opp alle kuler etter følgande reglar:

1. Den første kula hengast øverst på treet.
2. Det er plass til to kuler rett under kvar kule.
3. Kuler under ein foreldrekule plasserast til venstre for foreldrekula om hex-verdien på kula er lågare eller lik, og til høgre om verdien er større (oversatt til titallsystemet).
4. Om det ikkje er ledig plass under ei kule, blir den aktuelle kula plassert under den kula som allereide har plassen, og følger regel 3.

## Oppgåve:
Alle kulene ligg i rekkefølga dei plukkast opp i [denne fila](src/kuler.txt). Halvor si favorittkule er `#811A89`, og Alvhild si er `#8EAA54`. Kva for ei kule er næraste felles "foreldrekule" for begge favorittane? Oppgi hex-koden på denne kula.

## Døme:
For kulene:

```
#61FA11
#575947
#6E41C1
#0CD1AD
#F0C38B
#C9D2E0
#F515A7
#7E662A
#0189DA
#CBA3C2
#2FCDAA
#E5E81E
#E06162
#5176F2
#7D99B6
```
og favortittane `#7D99B6` og `#E06162` ser vi at næraste felles foreldrekule er `#C9D2E0`.

```
                                ___#61FA11___                                                            
                               /             \                                                           
           _________________#575947       #6E41C1_______________________________________           
          /                                                                             \          
    ___#0CD1AD___                                             ________________________#F0C38B___    
   /             \                                           /                                  \   
#0189DA       #2FCDAA___                               ___#C9D2E0___                         #F515A7
                        \                             /             \                               
                     #5176F2                    ___#7E662A       #CBA3C2__________                  
                                               /                                  \                 
                                           #7D99B6                         ___#E5E81E              
                                                                         /                        
                                                                     #E06162                     
```
