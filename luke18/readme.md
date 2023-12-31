# Findre Sinnes

Det har vært høy inflasjon på Nordpolen dette året. Leketøyproduksjon er dessverre svært dyrt og inflasjonen gjør det dyrere enn hva alvene har råd til! Dermed har Findre Sinnes bestemt seg for å kjøpe og selge aksjer for å få råd til nok materialer.

Findre ble litt for gira på aksjehandlingen og mistet helt oversikt. Dette gjorde han selvfølgelig bak ryggen til sin kone, Sola Ernberg, og alle de andre alvene. I [denne filen](graphs.zip) finner du samtlige grafer, med kjøpe- og selge punkter. Loggen viser hver time for de 26 siste arbeidsukene (altså 1040 timer). Kan du finne ut hvor mye Findre har tjent eller tapt?

## Eksempel
Her er et lite utsnitt av en graf, samt siste linje.

```
pris      +------------------------------------------------
      120 |           #                   #
      119 |        ### S    ####         # #
      118 |       #     #  #    K       #   #         S
      117 |    #K#       ##      ###   #     #   ##### #
      116 |#K #                     ###       # #       ##S
      115 |  #                                 #           
          +------------------------------------------------
      time 123456789...
      2086, 2510, 2468, 3471, 3397, 2202
```

Y-aksen representerer pris, og X-aksen representerer tid. I grafene vil du finne punkter hvor det står K eller S. Ved K punktene så har Findre kjøpt. Ved S punktene har Findre solgt. Prisen går fra 0 (linjen over tallene) til 150 (øverste linje). På bunnen av grafen (nederste linje) så finner du en sekvens av kommaseparerte tall med hvor mange aksjer som ble kjøpt og solgt i de forskjellige punktene. Helt på slutten selger Findre alltid alle aksjene han har, representert med ved en S på slutten av grafen. Det siste tallet i sekvensen er da antall resterende aksjer.
Gitt grafen og tallene over så har Findre tjent 4746kr:

`- 2086*116 - 2510*117 + 2468*119 - 3471*118 + 3397*118 + 2202*116 = 4746`

Svar med antall kroner i minus eller pluss, for eksempel `-3457` eller `4746`.
