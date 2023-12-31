# Kredittkortforfalskning

En luhn aften så erstattet Grinchen noen av alvene sine kredittkort med forfalskninger! Heldigvis finnes det en metode for å sjekke om et kort er falskt eller ekte.

## Hvordan sjekke validitet
Et kort har 24 siffer på nordpolen.

1. Sett av de to siste siffrene og gjør de om til et tall. Videre opererer vi kun med de 22 første sifrene.
2. Doble annenhvert siffer.
3. Summer tallene. Vi kaller dette resultatet for S.
4. Ta `(24 - (S modulo 24)) modulo 24` og sjekk om det blir likt tallet du lagde i steg 1.
5. Hvis de er like så er kortet gyldig.

## Eksempel
```
Vi har kredittkortet: 234671631264987292370142
1. Vi sitter igjen med 2346716312649872923701 og 42
2. Vi deler opp sifrene hver for seg og dobler annenhvert tall:
4, 3, 8, 6, 14, 1, 12, 3, 2, 2, 12, 4, 18, 8, 14, 2, 18, 2, 6, 7, 0, 1
3. Vi summerer alle tallene fra steg 2, som da blir 147
4. Vi tar (24 - (147 modulo 24)) modulo 24 = 21
5. 21 er ikke lik 42, og vi har derfor ikke et gyldig kort.
```

## Oppgave
Kan du finne ut hvor mange kort som Grinchen har byttet i [denne listen](src/kredittkort.txt) av kredittkort?
