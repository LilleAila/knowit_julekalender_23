# Historisk mail-bommert

Å nei! Historikkeralven klarte ved et uhell å åpne opp en ondsinnet .pdf-fil på e-mailen sin, og diktet han var i ferd med å levere til Julenissen har blitt kryptert.

Heldigvis har hackerene som sendte PDF-filen vært uvarsomme nok til å ikke slette [nøkkelfilen](src/input.txt), så cyberalvene tror de har mulighet til å dekryptere [diktet](src/cypher.txt). De er litt usikre på nøyaktig hvordan filen er kryptert, men det ser ut som om nøkkelfilen inneholder lister med tall som går fra 0-28 og er nøyaktig 29 elementer lang. Det kan også virke som om , og linjeneskiftene er ivaretatt.

Cyberalvene antar at dersom man har nøkkelen

```
  a   b   c  d   e   f  g   h  i   j  k  l   m  n   o   p  q   r   s   t  u   v   w  x   y  z   æ   ø   å
[15, 23, 21, 5, 11, 26, 8, 20, 6, 28, 2, 7, 19, 3, 22, 14, 24, 1, 18, 13, 4, 12, 27, 0, 17, 9, 25, 16, 10]
```
Vil ordet "Jul" bli til "åeh".

Visualisering av nøkkelkorrespondans:

```
  a   b   c  d   e   f  g   h  i   j  k  l   m  n   o   p  q   r   s   t  u   v   w  x   y  z   æ   ø   å
[ p,  x,  v, f,  l,  æ, i,  u, g,  å, c, h,  t, d,  w,  o,  y, b,  s,  n, e,  m,  ø, a,  r, j,  z,  q,  k]
```
